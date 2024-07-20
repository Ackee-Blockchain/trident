use super::light_client::TridentAccount;
use solana_program::instruction::InstructionError;
use solana_program_runtime::solana_rbpf::aligned_memory::{AlignedMemory, Pod};
use solana_program_runtime::solana_rbpf::ebpf::HOST_ALIGN;
use solana_sdk::entrypoint::{BPF_ALIGN_OF_U128, MAX_PERMITTED_DATA_INCREASE};
use solana_sdk::instruction::Instruction;
use std::cell::RefCell;
use std::collections::HashMap;
use std::mem::size_of;
use std::rc::Rc;
use std::slice::from_raw_parts_mut;

use solana_program::entrypoint::NON_DUP_MARKER;
use solana_sdk::instruction::AccountMeta;
use solana_sdk::{account_info::AccountInfo, pubkey::Pubkey};

pub(crate) struct SerializerCustomLight {
    buffer: AlignedMemory<HOST_ALIGN>,
}

impl SerializerCustomLight {
    pub fn new(size: usize) -> SerializerCustomLight {
        SerializerCustomLight {
            buffer: AlignedMemory::with_capacity(size),
        }
    }

    pub fn fill_write(&mut self, num: usize, value: u8) -> std::io::Result<()> {
        self.buffer.fill_write(num, value)
    }

    pub fn write<T: Pod>(&mut self, value: T) {
        // Safety:
        // in serialize_parameters_(aligned|unaligned) first we compute the
        // required size then we write into the newly allocated buffer. There's
        // no need to check bounds at every write.
        //
        // AlignedMemory::write_unchecked _does_ debug_assert!() that the capacity
        // is enough, so in the unlikely case we introduce a bug in the size
        // computation, tests will abort.
        unsafe {
            self.buffer.write_unchecked(value);
        }
    }

    pub fn write_all(&mut self, value: &[u8]) {
        // Safety:
        // see write() - the buffer is guaranteed to be large enough
        unsafe {
            self.buffer.write_all_unchecked(value);
        }
    }

    pub fn write_account_custom(
        &mut self,
        account: &TridentAccount,
    ) -> Result<(), InstructionError> {
        self.write_all(&account.data);
        let align_offset = (account.data.len() as *const u8).align_offset(BPF_ALIGN_OF_U128);
        self.fill_write(MAX_PERMITTED_DATA_INCREASE + align_offset, 0)
            .map_err(|_| InstructionError::InvalidArgument)?;

        Ok(())
    }

    pub fn finish(self) -> AlignedMemory<HOST_ALIGN> {
        self.buffer
    }
}

pub(crate) fn serialize_accounts_custom<'a>(
    instruction: &'a Instruction,
    de_duplicate_accounts: &'a HashMap<Pubkey, usize>,
    trident_accounts: &'a [Option<&TridentAccount>],
) -> AlignedMemory<HOST_ALIGN> {
    // Obtain Number of duplicate accounts
    // ie Number of all Accounts - number of Unique Accounts
    let duplicate_accounts_count = instruction.accounts.len() - de_duplicate_accounts.len();

    let mut size = size_of::<u64>();

    // Duplicate accounts are represented by 1 byte duplicate flag plus 7 padding bytes to 64-aligned.
    size += 8 * duplicate_accounts_count;

    for trident_account in trident_accounts.iter() {
        let data_len = match trident_account {
            Some(t_account) => t_account.data.len(),
            None => 0,
        };

        // This block is 64-bit aligned
        size += size_of::<u8>() // duplicate flag
                + size_of::<u8>() // is_signer
                + size_of::<u8>() // is_writable
                + size_of::<u8>() // executable
                + size_of::<u32>() // original_data_len
                + size_of::<Pubkey>()  // key
                + size_of::<Pubkey>() // owner
                + size_of::<u64>()  // lamports
                + size_of::<u64>()  // data len
                + MAX_PERMITTED_DATA_INCREASE
                + size_of::<u64>(); // rent epoch
        size += data_len + (data_len as *const u8).align_offset(BPF_ALIGN_OF_U128);
    }

    let mut s = SerializerCustomLight::new(size);

    s.write::<u64>((instruction.accounts.len() as u64).to_le());
    for (i, (instruction_account_meta, trident_account)) in instruction
        .accounts
        .iter()
        .zip(trident_accounts)
        .enumerate()
    {
        // We can unwrap, it should never be None.
        let position = de_duplicate_accounts
            .get(&instruction_account_meta.pubkey)
            .unwrap();

        if i == *position {
            // first occurence of the account
            match trident_account {
                Some(account) => {
                    s.write::<u8>(NON_DUP_MARKER);
                    s.write::<u8>(instruction_account_meta.is_signer as u8);
                    s.write::<u8>(instruction_account_meta.is_writable as u8);
                    s.write::<u8>(account.executable as u8);
                    s.write_all(&[0u8, 0, 0, 0]);
                    s.write_all(instruction_account_meta.pubkey.as_ref());
                    s.write_all(account.owner.as_ref());
                    s.write::<u64>(account.lamports.to_le());
                    s.write::<u64>((account.data.len() as u64).to_le());
                    s.write_account_custom(account).unwrap();
                    s.write::<u64>((account.rent_epoch).to_le());
                }
                None => {
                    let account = TridentAccount::default();
                    s.write::<u8>(NON_DUP_MARKER);
                    s.write::<u8>(instruction_account_meta.is_signer as u8);
                    s.write::<u8>(instruction_account_meta.is_writable as u8);
                    s.write::<u8>(account.executable as u8);
                    s.write_all(&[0u8, 0, 0, 0]);
                    s.write_all(instruction_account_meta.pubkey.as_ref());
                    s.write_all(account.owner.as_ref());
                    s.write::<u64>(account.lamports.to_le());
                    s.write::<u64>((account.data.len() as u64).to_le());
                    s.write_account_custom(&account).unwrap();
                    s.write::<u64>((account.rent_epoch).to_le());
                }
            };
        } else {
            // it is a duplicate
            s.write::<u8>(*position as u8);
            s.write_all(&[0u8, 0, 0, 0, 0, 0, 0]);
        }
    }

    s.finish()
}

pub(crate) fn get_duplicate_accounts(
    instruction_account_metas: &mut [AccountMeta],
) -> HashMap<Pubkey, usize> {
    let mut dedup_ixs: HashMap<Pubkey, usize> =
        HashMap::with_capacity(instruction_account_metas.len());

    for i in 0..instruction_account_metas.len() {
        match dedup_ixs.get(&instruction_account_metas[i].pubkey) {
            // the account is already in the HashMap, so it is a duplicate
            Some(&reference_index) => {
                //  if the duplicate account is signer or writable, make sure also the referrence account is set accordingly
                if instruction_account_metas[i].is_signer {
                    instruction_account_metas[reference_index].is_signer = true;
                }
                if instruction_account_metas[i].is_writable {
                    instruction_account_metas[reference_index].is_writable = true;
                }
            }
            // the account is not yet in the HashMap, so it is not a duplicate
            None => {
                dedup_ixs.insert(instruction_account_metas[i].pubkey, i);
            }
        };
    }
    dedup_ixs
}

// REF
// https://github.com/solana-labs/solana/blob/27eff8408b7223bb3c4ab70523f8a8dca3ca6645/sdk/program/src/entrypoint.rs#L277
/// Deserialize the input arguments
///
/// The integer arithmetic in this method is safe when called on a buffer that was
/// serialized by Trident. Use with buffers serialized otherwise is unsupported and
/// done at one's own risk.
///
/// # Safety
pub(crate) unsafe fn deserialize_custom<'a>(input: *mut u8) -> Vec<AccountInfo<'a>> {
    let mut offset: usize = 0;

    // Number of accounts present

    #[allow(clippy::cast_ptr_alignment)]
    let num_accounts = *(input.add(offset) as *const u64) as usize;
    offset += size_of::<u64>();

    // Account Infos

    let mut accounts = Vec::with_capacity(num_accounts);
    for _ in 0..num_accounts {
        let dup_info = *(input.add(offset) as *const u8);
        offset += size_of::<u8>();
        if dup_info == NON_DUP_MARKER {
            #[allow(clippy::cast_ptr_alignment)]
            let is_signer = *(input.add(offset) as *const u8) != 0;
            offset += size_of::<u8>();

            #[allow(clippy::cast_ptr_alignment)]
            let is_writable = *(input.add(offset) as *const u8) != 0;
            offset += size_of::<u8>();

            #[allow(clippy::cast_ptr_alignment)]
            let executable = *(input.add(offset) as *const u8) != 0;
            offset += size_of::<u8>();

            // The original data length is stored here because these 4 bytes were
            // originally only used for padding and served as a good location to
            // track the original size of the account data in a compatible way.
            let original_data_len_offset = offset;
            offset += size_of::<u32>();

            let key: &Pubkey = &*(input.add(offset) as *const Pubkey);
            offset += size_of::<Pubkey>();

            let owner: &Pubkey = &*(input.add(offset) as *const Pubkey);
            offset += size_of::<Pubkey>();

            #[allow(clippy::cast_ptr_alignment)]
            let lamports = Rc::new(RefCell::new(&mut *(input.add(offset) as *mut u64)));
            offset += size_of::<u64>();

            #[allow(clippy::cast_ptr_alignment)]
            let data_len = *(input.add(offset) as *const u64) as usize;
            offset += size_of::<u64>();

            // Store the original data length for detecting invalid reallocations and
            // requires that MAX_PERMITTED_DATA_LENGTH fits in a u32
            *(input.add(original_data_len_offset) as *mut u32) = data_len as u32;

            let data = Rc::new(RefCell::new({
                from_raw_parts_mut(input.add(offset), data_len)
            }));
            offset += data_len + MAX_PERMITTED_DATA_INCREASE;
            offset += (offset as *const u8).align_offset(BPF_ALIGN_OF_U128); // padding

            #[allow(clippy::cast_ptr_alignment)]
            let rent_epoch = *(input.add(offset) as *const u64);
            offset += size_of::<u64>();

            accounts.push(AccountInfo {
                key,
                is_signer,
                is_writable,
                lamports,
                data,
                owner,
                executable,
                rent_epoch,
            });
        } else {
            offset += 7; // padding

            // Duplicate account, clone the original
            accounts.push(accounts[dup_info as usize].clone());
        }
    }

    accounts
}
