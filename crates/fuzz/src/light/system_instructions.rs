use solana_program::program_error::ProgramError;
#[allow(deprecated)]
use solana_sdk::program_stubs;
use solana_sdk::{
    account_info::AccountInfo, pubkey::Pubkey, system_instruction::MAX_PERMITTED_DATA_LENGTH,
    system_program,
};

pub(crate) trait SystemInstructionsTrident: program_stubs::SyscallStubs {
    fn create_account(
        &self,
        from: &AccountInfo,
        to: &AccountInfo,
        space: u64,
        owner: Pubkey,
        lamports: u64,
    ) -> Result<(), ProgramError> {
        {
            if to.lamports() > 0 {
                self.sol_log(&format!(
                    "Create Account: account {:?} already in use",
                    to.key
                ));
                return Err(ProgramError::AccountAlreadyInitialized);
            }

            self.allocate_and_assign(to, space, owner)?;
        }
        self.transfer(from, to, lamports)
    }
    fn allocate_and_assign(
        &self,
        to: &AccountInfo,
        space: u64,
        owner: Pubkey,
    ) -> Result<(), ProgramError> {
        self.allocate(to, space)?;
        self.assign(to, owner)
    }
    fn allocate(&self, to: &AccountInfo, space: u64) -> Result<(), ProgramError> {
        if !to.is_signer {
            self.sol_log(&format!("Allocate: 'to' account {:?} must sign", to.key));
            return Err(ProgramError::MissingRequiredSignature);
        }
        // if it looks like the `to` account is already in use, bail
        //   (note that the id check is also enforced by message_processor)
        if !to.data_is_empty() || !system_program::check_id(to.owner) {
            self.sol_log(&format!("Allocate: account {:?} already in use", to.key));
            return Err(ProgramError::AccountAlreadyInitialized);
        }
        if space > MAX_PERMITTED_DATA_LENGTH {
            self.sol_log(&format!(
                "Allocate: requested {}, max allowed {}",
                space, MAX_PERMITTED_DATA_LENGTH
            ));
            return Err(ProgramError::InvalidAccountData);
        }

        // TODO
        // https://github.com/solana-labs/solana/blob/27eff8408b7223bb3c4ab70523f8a8dca3ca6645/programs/system/src/system_processor.rs#L107
        to.realloc(space as usize, false)
    }
    fn assign(&self, to: &AccountInfo, new_owner: Pubkey) -> Result<(), ProgramError> {
        // no work to do, just return
        if *to.owner == new_owner {
            return Ok(());
        }

        if !to.is_signer {
            self.sol_log(&format!("Assign: account {:?} must sign", to.key));
            return Err(ProgramError::MissingRequiredSignature);
        }

        // TODO
        // https://github.com/solana-labs/solana/blob/27eff8408b7223bb3c4ab70523f8a8dca3ca6645/programs/system/src/system_processor.rs#L129
        to.assign(&new_owner);
        Ok(())
    }
    fn transfer(
        &self,
        from: &AccountInfo,
        to: &AccountInfo,
        lamports: u64,
    ) -> Result<(), ProgramError> {
        if !from.is_signer {
            self.sol_log(&format!("Transfer: `from` account {} must sign", from.key));
            return Err(ProgramError::MissingRequiredSignature);
        }
        self.transfer_verified(from, to, lamports)
    }

    fn transfer_verified(
        &self,
        from: &AccountInfo,
        to: &AccountInfo,
        lamports: u64,
    ) -> Result<(), ProgramError> {
        if !from.data_is_empty() {
            self.sol_log("Transfer: `from` must not carry data");
            return Err(ProgramError::InvalidArgument);
        }
        if lamports > from.lamports() {
            self.sol_log(&format!(
                "Transfer: insufficient lamports {}, need {}",
                from.lamports(),
                lamports,
            ));
            return Err(ProgramError::InsufficientFunds);
        }

        // TODO https://github.com/solana-labs/solana/blob/27eff8408b7223bb3c4ab70523f8a8dca3ca6645/programs/system/src/system_processor.rs#L206
        subtract_lamports(from, lamports);
        add_lamports(to, lamports);
        Ok(())
    }
    fn create_address(
        &self,
        address: &Pubkey,
        base: &Pubkey,
        seed: &str,
        owner: &Pubkey,
    ) -> Result<(), ProgramError> {
        let address_with_seed = Pubkey::create_with_seed(base, seed, owner)?;
        // re-derive the address, must match the supplied address
        if *address != address_with_seed {
            self.sol_log(&format!(
                "Create: address {} does not match derived address {}",
                address, address_with_seed
            ));
            return Err(ProgramError::InvalidSeeds);
        }
        Ok(())
    }

    fn transfer_with_seed(
        &self,
        from: &AccountInfo,
        from_base: &AccountInfo,
        to: &AccountInfo,
        from_seed: &str,
        from_owner: &Pubkey,
        lamports: u64,
    ) -> Result<(), ProgramError> {
        if !from_base.is_signer {
            self.sol_log(&format!(
                "Transfer: 'from' account {:?} must sign",
                from_base.key
            ));
            return Err(ProgramError::MissingRequiredSignature);
        }
        self.create_address(from.key, from_base.key, from_seed, from_owner)?;

        self.transfer_verified(from, to, lamports)
    }
}

fn subtract_lamports(from: &AccountInfo, lamports: u64) {
    let from_lamports = from.lamports();
    match from_lamports.checked_sub(lamports) {
        Some(new_balance) => {
            let mut mutable_lamports = from
                .try_borrow_mut_lamports()
                .expect("From: cannot borrow mutable lamports");
            **mutable_lamports = new_balance;
        }
        None => {
            panic!("From: not enough lamports")
        }
    }
}

fn add_lamports(to: &AccountInfo, lamports: u64) {
    let to_lamports = to.lamports();
    match to_lamports.checked_add(lamports) {
        Some(new_balance) => {
            let mut mutable_lamports = to
                .try_borrow_mut_lamports()
                .expect("To: cannot borrow mutable lamports");
            **mutable_lamports = new_balance;
        }
        None => {
            panic!("To: lamports addition overflow")
        }
    }
}
