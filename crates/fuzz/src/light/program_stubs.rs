use std::sync::{Arc, RwLock};

use solana_program::entrypoint::ProgramResult;
use solana_sdk::{
    account_info::AccountInfo, instruction::Instruction, program_stubs::SyscallStubs,
    pubkey::Pubkey, system_instruction::SystemInstruction,
};

use solana_program::program_error::ProgramError;
#[allow(deprecated)]
use solana_sdk::program_stubs;

use super::clock::Clock;
use super::light_client::get_light_client;

use super::system_instructions::SystemInstructionsTrident;

struct TestSyscallStubs {
    pub callers: Arc<RwLock<Vec<Pubkey>>>,
    pub data: Arc<RwLock<TransactionReturnData>>,
}

impl SystemInstructionsTrident for TestSyscallStubs {}

impl TestSyscallStubs {
    fn add_caller_n_get_signers(
        &self,
        program_id: Pubkey,
        signers_seeds: &[&[&[u8]]],
    ) -> Vec<Pubkey> {
        let signers;
        {
            let mut callers = self.callers.write().unwrap();
            let caller = *callers.last().unwrap();
            callers.push(program_id);

            signers = signers_seeds
                .iter()
                .map(|seeds| Pubkey::create_program_address(seeds, &caller).unwrap())
                .collect::<Vec<_>>();
        }
        signers
    }
    fn clean_caller(&self) {
        let mut callers = self.callers.write().unwrap();
        callers.pop();
    }
    fn update_account_infos<'a>(
        &self,
        old_account_infos: &[AccountInfo<'a>],
        signers: &[Pubkey],
        instruction: &Instruction,
    ) -> Vec<AccountInfo<'a>> {
        let mut new_account_infos = vec![];

        for meta in instruction.accounts.iter() {
            for account_info in old_account_infos.iter() {
                if meta.pubkey == *account_info.key {
                    let mut new_account_info = account_info.clone();
                    for signer in signers.iter() {
                        if *account_info.key == *signer {
                            new_account_info.is_signer = true;
                        }
                    }
                    new_account_infos.push(new_account_info);
                    break;
                }
            }
        }
        new_account_infos
    }
}

#[derive(Default)]
pub struct TransactionReturnData {
    pub program_id: Pubkey,
    pub data: Vec<u8>,
}

/// Assert that enough accounts were supplied to this Instruction
pub fn check_number_of_instruction_accounts(
    account_infos: &[AccountInfo],
    expected_at_least: u8,
) -> Result<(), ProgramError> {
    if account_infos.len() < expected_at_least.into() {
        Err(ProgramError::NotEnoughAccountKeys)
    } else {
        Ok(())
    }
}

impl SyscallStubs for TestSyscallStubs {
    fn sol_log(&self, message: &str) {
        println!("{message}"); // FIXME maybe eprintln?
    }
    fn sol_get_rent_sysvar(&self, _var_addr: *mut u8) -> u64 {
        0
    }
    fn sol_log_compute_units(&self) {
        self.sol_log("SyscallStubs: sol_log_compute_units() not available");
    }
    fn sol_invoke_signed(
        &self,
        instruction: &Instruction,
        account_infos: &[AccountInfo],
        signers_seeds: &[&[&[u8]]],
    ) -> ProgramResult {
        let signers = &self.add_caller_n_get_signers(instruction.program_id, signers_seeds);

        let new_account_infos = self.update_account_infos(account_infos, signers, instruction);

        if instruction.program_id == solana_program::system_program::ID {
            let instruction =
                solana_sdk::program_utils::limited_deserialize(&instruction.data).unwrap();
            match instruction {
                SystemInstruction::CreateAccount {
                    lamports,
                    space,
                    owner,
                } => {
                    // https://github.com/solana-labs/solana/blob/27eff8408b7223bb3c4ab70523f8a8dca3ca6645/programs/system/src/system_processor.rs#L310
                    self.sol_log("Processing CreateAccount");

                    let res = {
                        check_number_of_instruction_accounts(account_infos, 2)?;

                        let from = &new_account_infos[0];
                        let to = &new_account_infos[1];

                        self.create_account(from, to, space, owner, lamports)
                    };

                    self.clean_caller();
                    res
                }
                SystemInstruction::CreateAccountWithSeed {
                    base,
                    seed,
                    lamports,
                    space,
                    owner,
                } => {
                    // https://github.com/solana-labs/solana/blob/27eff8408b7223bb3c4ab70523f8a8dca3ca6645/programs/system/src/system_processor.rs#L336
                    self.sol_log("Processing CreateAccountWithSeed");

                    let res = {
                        check_number_of_instruction_accounts(&new_account_infos, 2)?;

                        let from = &new_account_infos[0];
                        let to = &new_account_infos[1];

                        self.create_address(to.key, &base, &seed, &owner)?;
                        self.create_account(from, to, space, owner, lamports)
                    };

                    self.clean_caller();
                    res
                }
                SystemInstruction::Assign { owner } => {
                    // https://github.com/solana-labs/solana/blob/27eff8408b7223bb3c4ab70523f8a8dca3ca6645/programs/system/src/system_processor.rs#L364
                    self.sol_log("Processing Assign");

                    let res = {
                        check_number_of_instruction_accounts(&new_account_infos, 1)?;

                        let to = &new_account_infos[0];

                        self.assign(to, owner)
                    };

                    self.clean_caller();
                    res
                }
                SystemInstruction::Transfer { lamports } => {
                    // https://github.com/solana-labs/solana/blob/27eff8408b7223bb3c4ab70523f8a8dca3ca6645/programs/system/src/system_processor.rs#L377
                    self.sol_log("Processing Transfer");

                    let res = {
                        check_number_of_instruction_accounts(&new_account_infos, 2)?;
                        let from = &new_account_infos[0];
                        let to = &new_account_infos[1];
                        self.transfer(from, to, lamports)
                    };

                    self.clean_caller();
                    res
                }
                SystemInstruction::TransferWithSeed {
                    lamports,
                    from_seed,
                    from_owner,
                } => {
                    // https://github.com/solana-labs/solana/blob/27eff8408b7223bb3c4ab70523f8a8dca3ca6645/programs/system/src/system_processor.rs#L388
                    self.sol_log("Processing TransferWithSeed");

                    let res = {
                        check_number_of_instruction_accounts(&new_account_infos, 3)?;

                        let from = &new_account_infos[0];
                        let from_base = &new_account_infos[1];
                        let to = &new_account_infos[2];

                        self.transfer_with_seed(
                            from,
                            from_base,
                            to,
                            &from_seed,
                            &from_owner,
                            lamports,
                        )
                    };

                    self.clean_caller();
                    res
                }
                SystemInstruction::AdvanceNonceAccount => {
                    // https://github.com/solana-labs/solana/blob/27eff8408b7223bb3c4ab70523f8a8dca3ca6645/programs/system/src/system_processor.rs#L406
                    self.sol_log("Processing AdvanceNonceAccount");
                    self.clean_caller();

                    Ok(())
                }
                SystemInstruction::WithdrawNonceAccount(_lamports) => {
                    // https://github.com/solana-labs/solana/blob/27eff8408b7223bb3c4ab70523f8a8dca3ca6645/programs/system/src/system_processor.rs#L425
                    self.sol_log("Processing WithdrawNonceAccount");
                    self.clean_caller();

                    Ok(())
                }
                SystemInstruction::InitializeNonceAccount(_authorized) => {
                    // https://github.com/solana-labs/solana/blob/27eff8408b7223bb3c4ab70523f8a8dca3ca6645/programs/system/src/system_processor.rs#L445
                    self.sol_log("Processing InitializeNonceAccount");
                    self.clean_caller();

                    Ok(())
                }
                SystemInstruction::AuthorizeNonceAccount(_nonce_authority) => {
                    // https://github.com/solana-labs/solana/blob/27eff8408b7223bb3c4ab70523f8a8dca3ca6645/programs/system/src/system_processor.rs#L465
                    self.sol_log("Processing AuthorizeNonceAccount");
                    self.clean_caller();

                    Ok(())
                }
                SystemInstruction::UpgradeNonceAccount => {
                    // https://github.com/solana-labs/solana/blob/27eff8408b7223bb3c4ab70523f8a8dca3ca6645/programs/system/src/system_processor.rs#L471
                    self.sol_log("Processing UpgradeNonceAccount");
                    self.clean_caller();

                    Ok(())
                }
                SystemInstruction::Allocate { space } => {
                    // https://github.com/solana-labs/solana/blob/27eff8408b7223bb3c4ab70523f8a8dca3ca6645/programs/system/src/system_processor.rs#L487
                    self.sol_log("Processing Allocate");

                    let res = {
                        check_number_of_instruction_accounts(&new_account_infos, 1)?;

                        let to = &new_account_infos[0];

                        self.allocate(to, space)
                    };
                    self.clean_caller();
                    res
                }
                SystemInstruction::AllocateWithSeed {
                    base,
                    seed,
                    space,
                    owner,
                } => {
                    // https://github.com/solana-labs/solana/blob/27eff8408b7223bb3c4ab70523f8a8dca3ca6645/programs/system/src/system_processor.rs#L500
                    self.sol_log("Processing AllocateWithSeed");

                    let res = {
                        check_number_of_instruction_accounts(&new_account_infos, 1)?;

                        let to = &new_account_infos[0];

                        self.create_address(to.key, &base, &seed, &owner)?;
                        self.allocate_and_assign(to, space, owner)
                    };

                    self.clean_caller();
                    res
                }
                SystemInstruction::AssignWithSeed { base, seed, owner } => {
                    // https://github.com/solana-labs/solana/blob/27eff8408b7223bb3c4ab70523f8a8dca3ca6645/programs/system/src/system_processor.rs#L525
                    self.sol_log("Processing AssignWithSeed");

                    let res = {
                        check_number_of_instruction_accounts(&new_account_infos, 1)?;

                        let to = &new_account_infos[0];
                        self.create_address(to.key, &base, &seed, &owner)?;
                        self.assign(to, owner)
                    };

                    self.clean_caller();
                    res
                }
            }
        } else {
            let client = get_light_client();

            match client.programs.get(&instruction.program_id) {
                Some(process_function) => {
                    let res = process_function(
                        &instruction.program_id,
                        &new_account_infos,
                        &instruction.data,
                    );
                    self.clean_caller();
                    res
                }
                None => {
                    let message = format!(
                        "SyscallStubs: sol_invoke_signed() for {} not available",
                        instruction.program_id
                    );
                    self.sol_log(&message);
                    Ok(())
                }
            }
        }
    }
    fn sol_get_clock_sysvar(&self, var_addr: *mut u8) -> u64 {
        let now = Clock::now();
        unsafe {
            *(var_addr as *mut _ as *mut Clock) = Clock::clone(&now);
            0
        }
    }

    fn sol_set_return_data(&self, _data: &[u8]) {
        let mut data = self.data.write().unwrap();
        let caller = self.callers.read().unwrap();
        let caller = caller.last().unwrap();
        let d = TransactionReturnData {
            program_id: *caller,
            data: _data.to_vec(),
        };
        *data = d;
    }

    fn sol_get_return_data(&self) -> Option<(Pubkey, Vec<u8>)> {
        let data = self.data.read().ok()?;
        let (program_id, data) = (data.program_id, data.data.to_owned());
        Some((program_id, data))
    }

    fn sol_get_stack_height(&self) -> u64 {
        let callers = self.callers.read().unwrap();
        callers.len() as u64
    }
}

#[allow(dead_code)]
pub fn test_syscall_stubs(program_id: Pubkey) {
    use std::sync::Once;
    static ONCE: Once = Once::new();

    ONCE.call_once(|| {
        program_stubs::set_syscall_stubs(Box::new(TestSyscallStubs {
            callers: Arc::new(RwLock::new(vec![program_id])), // FIX the first caller does not have to be the user program
            data: Arc::new(RwLock::new(TransactionReturnData::default())),
        }));
    });
}
