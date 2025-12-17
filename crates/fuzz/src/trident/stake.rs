use borsh::BorshDeserialize;
use solana_sdk::account::ReadableAccount;
use solana_sdk::instruction::Instruction;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::rent;
use solana_stake_interface::instruction::LockupArgs;
use solana_stake_interface::state::Authorized;
use solana_stake_interface::state::Lockup;
use solana_stake_interface::state::StakeAuthorize;
use solana_stake_interface::state::StakeStateV2;

use crate::trident::Trident;

impl Trident {
    /// Creates instructions to create and delegate a stake account
    ///
    /// Generates instructions to create a new stake account and immediately delegate it to the specified
    /// vote account, combining both operations.
    ///
    /// # Arguments
    /// * `from_pubkey` - The public key of the account funding the stake account creation
    /// * `stake_pubkey` - The public key of the stake account to create
    /// * `vote_pubkey` - The public key of the vote account to delegate to
    /// * `authorized` - The authorized staker and withdrawer authorities
    /// * `lockup` - The lockup configuration for the stake account
    /// * `lamports` - The number of lamports to transfer to the stake account
    ///
    /// # Returns
    /// A vector of instructions that need to be executed with `process_transaction`
    pub fn create_and_delegate_account(
        &mut self,
        from_pubkey: &Pubkey,
        stake_pubkey: &Pubkey,
        vote_pubkey: &Pubkey,
        authorized: &Authorized,
        lockup: Lockup,
        lamports: u64,
    ) -> Vec<Instruction> {
        solana_stake_interface::instruction::create_account_and_delegate_stake(
            from_pubkey,
            stake_pubkey,
            vote_pubkey,
            authorized,
            &lockup,
            lamports,
        )
    }

    /// Creates instructions to initialize a stake account without delegation
    ///
    /// Generates instructions to create a new stake account with the specified authorities and lockup
    /// configuration, but does not delegate it to any vote account. The stake account will be funded
    /// with the minimum rent-exempt balance.
    ///
    /// # Arguments
    /// * `payer` - The public key of the account funding the stake account creation
    /// * `stake_pubkey` - The public key of the stake account to create
    /// * `withdrawer_pubkey` - The public key of the withdrawer authority
    /// * `staker_pubkey` - The public key of the staker authority
    /// * `unix_timestamp` - The Unix timestamp before which the stake cannot be withdrawn
    /// * `epoch` - The epoch before which the stake cannot be withdrawn
    /// * `custodian` - The public key of the custodian who can modify the lockup
    ///
    /// # Returns
    /// A vector of instructions that need to be executed with `process_transaction`
    #[allow(clippy::too_many_arguments)]
    pub fn create_initialized_account(
        &mut self,
        payer: &Pubkey,
        stake_pubkey: &Pubkey,
        withdrawer_pubkey: &Pubkey,
        staker_pubkey: &Pubkey,
        unix_timestamp: i64,
        epoch: u64,
        custodian: &Pubkey,
    ) -> Vec<Instruction> {
        let minimum_rent = rent::Rent::default()
            .minimum_balance(solana_stake_interface::state::StakeStateV2::size_of());
        let authorized = Authorized {
            staker: *staker_pubkey,
            withdrawer: *withdrawer_pubkey,
        };

        let lockup = Lockup {
            unix_timestamp,
            epoch,
            custodian: *custodian,
        };
        solana_stake_interface::instruction::create_account(
            payer,
            stake_pubkey,
            &authorized,
            &lockup,
            minimum_rent,
        )
    }

    /// Creates instructions to split a stake account
    ///
    /// Generates instructions to split a portion of the stake from an existing stake account
    /// into a new stake account.
    ///
    /// # Arguments
    /// * `stake_pubkey` - The public key of the source stake account to split from
    /// * `authorized_pubkey` - The public key of the stake authority
    /// * `lamports` - The number of lamports to split into the new stake account
    /// * `split_stake_pubkey` - The public key of the new stake account to receive the split stake
    ///
    /// # Returns
    /// A vector of instructions that need to be executed with `process_transaction`
    pub fn split_stake(
        &mut self,
        stake_pubkey: &Pubkey,
        authorized_pubkey: &Pubkey,
        lamports: u64,
        split_stake_pubkey: &Pubkey,
    ) -> Vec<Instruction> {
        solana_stake_interface::instruction::split(
            stake_pubkey,
            authorized_pubkey,
            lamports,
            split_stake_pubkey,
        )
    }

    /// Creates instructions to merge two stake accounts
    ///
    /// Generates instructions to merge a source stake account into a destination stake account.
    /// Both accounts must have the same authorized staker and withdrawer, and be in a compatible state.
    ///
    /// # Arguments
    /// * `destination_stake_pubkey` - The public key of the stake account to merge into
    /// * `source_stake_pubkey` - The public key of the stake account to merge from
    /// * `authorized_pubkey` - The public key of the stake authority
    ///
    /// # Returns
    /// A vector of instructions that need to be executed with `process_transaction`
    pub fn merge_stake(
        &mut self,
        destination_stake_pubkey: &Pubkey,
        source_stake_pubkey: &Pubkey,
        authorized_pubkey: &Pubkey,
    ) -> Vec<Instruction> {
        solana_stake_interface::instruction::merge(
            destination_stake_pubkey,
            source_stake_pubkey,
            authorized_pubkey,
        )
    }

    /// Creates an instruction to authorize a new staker or withdrawer
    ///
    /// Generates an instruction to change the staker or withdrawer authority of a stake account.
    ///
    /// # Arguments
    /// * `stake_pubkey` - The public key of the stake account
    /// * `authorized_pubkey` - The public key of the current authority being replaced
    /// * `new_authorized_pubkey` - The public key of the new authority
    /// * `stake_authorize` - The type of authorization to change (Staker or Withdrawer)
    /// * `custodian_pubkey` - Optional custodian required if the stake is locked
    ///
    /// # Returns
    /// An instruction that needs to be executed with `process_transaction`
    pub fn authorize(
        &mut self,
        stake_pubkey: &Pubkey,
        authorized_pubkey: &Pubkey,
        new_authorized_pubkey: &Pubkey,
        stake_authorize: StakeAuthorize,
        custodian_pubkey: Option<&Pubkey>,
    ) -> Instruction {
        solana_stake_interface::instruction::authorize(
            stake_pubkey,
            authorized_pubkey,
            new_authorized_pubkey,
            stake_authorize,
            custodian_pubkey,
        )
    }

    /// Creates an instruction to authorize a new staker or withdrawer with signature verification
    ///
    /// Generates an instruction to change the staker or withdrawer authority of a stake account,
    /// requiring the new authority to sign the transaction.
    ///
    /// # Arguments
    /// * `stake_pubkey` - The public key of the stake account
    /// * `authorized_pubkey` - The public key of the current authority being replaced
    /// * `new_authorized_pubkey` - The public key of the new authority (must sign)
    /// * `stake_authorize` - The type of authorization to change (Staker or Withdrawer)
    /// * `custodian_pubkey` - Optional custodian required if the stake is locked
    ///
    /// # Returns
    /// An instruction that needs to be executed with `process_transaction`
    pub fn authorize_checked(
        &mut self,
        stake_pubkey: &Pubkey,
        authorized_pubkey: &Pubkey,
        new_authorized_pubkey: &Pubkey,
        stake_authorize: StakeAuthorize,
        custodian_pubkey: Option<&Pubkey>,
    ) -> Instruction {
        solana_stake_interface::instruction::authorize_checked(
            stake_pubkey,
            authorized_pubkey,
            new_authorized_pubkey,
            stake_authorize,
            custodian_pubkey,
        )
    }

    /// Creates an instruction to delegate a stake account to a vote account
    ///
    /// Generates an instruction to delegate an initialized stake account to a validator's vote account.
    ///
    /// # Arguments
    /// * `stake_pubkey` - The public key of the stake account to delegate
    /// * `authorized_pubkey` - The public key of the stake authority
    /// * `vote_pubkey` - The public key of the vote account to delegate to
    ///
    /// # Returns
    /// An instruction that needs to be executed with `process_transaction`
    pub fn delegate_stake(
        &mut self,
        stake_pubkey: &Pubkey,
        authorized_pubkey: &Pubkey,
        vote_pubkey: &Pubkey,
    ) -> Instruction {
        solana_stake_interface::instruction::delegate_stake(
            stake_pubkey,
            authorized_pubkey,
            vote_pubkey,
        )
    }

    /// Creates an instruction to withdraw lamports from a stake account
    ///
    /// Generates an instruction to withdraw lamports from a stake account to a destination account.
    /// The stake must be deactivated and the cooldown period must have elapsed for full withdrawal.
    ///
    /// # Arguments
    /// * `stake_pubkey` - The public key of the stake account to withdraw from
    /// * `withdrawer_pubkey` - The public key of the withdrawer authority
    /// * `to_pubkey` - The public key of the destination account to receive the lamports
    /// * `lamports` - The number of lamports to withdraw
    /// * `custodian_pubkey` - Optional custodian required if the stake is locked
    ///
    /// # Returns
    /// An instruction that needs to be executed with `process_transaction`
    pub fn withdraw(
        &mut self,
        stake_pubkey: &Pubkey,
        withdrawer_pubkey: &Pubkey,
        to_pubkey: &Pubkey,
        lamports: u64,
        custodian_pubkey: Option<&Pubkey>,
    ) -> Instruction {
        solana_stake_interface::instruction::withdraw(
            stake_pubkey,
            withdrawer_pubkey,
            to_pubkey,
            lamports,
            custodian_pubkey,
        )
    }

    /// Creates an instruction to deactivate a stake account
    ///
    /// Generates an instruction to deactivate a delegated stake account. After deactivation,
    /// the stake will enter a cooldown period before it can be withdrawn.
    ///
    /// # Arguments
    /// * `stake_pubkey` - The public key of the stake account to deactivate
    /// * `authorized_pubkey` - The public key of the stake authority
    ///
    /// # Returns
    /// An instruction that needs to be executed with `process_transaction`
    pub fn deactivate_stake(
        &mut self,
        stake_pubkey: &Pubkey,
        authorized_pubkey: &Pubkey,
    ) -> Instruction {
        solana_stake_interface::instruction::deactivate_stake(stake_pubkey, authorized_pubkey)
    }

    /// Creates an instruction to set the lockup parameters of a stake account
    ///
    /// Generates an instruction to modify the lockup configuration of a stake account.
    /// Only the custodian can modify the lockup while it is active.
    ///
    /// # Arguments
    /// * `stake_pubkey` - The public key of the stake account
    /// * `unix_timestamp` - Optional new Unix timestamp for the lockup
    /// * `epoch` - Optional new epoch for the lockup
    /// * `custodian` - Optional new custodian public key
    /// * `custodian_pubkey` - The public key of the current custodian
    ///
    /// # Returns
    /// An instruction that needs to be executed with `process_transaction`
    pub fn set_lockup(
        &mut self,
        stake_pubkey: &Pubkey,
        unix_timestamp: Option<i64>,
        epoch: Option<u64>,
        custodian: Option<Pubkey>,
        custodian_pubkey: &Pubkey,
    ) -> Instruction {
        let lockup_args = LockupArgs {
            unix_timestamp,
            epoch,
            custodian,
        };
        solana_stake_interface::instruction::set_lockup(
            stake_pubkey,
            &lockup_args,
            custodian_pubkey,
        )
    }

    /// Creates an instruction to set the lockup parameters with signature verification
    ///
    /// Generates an instruction to modify the lockup configuration of a stake account,
    /// requiring the new custodian (if provided) to sign the transaction.
    ///
    /// # Arguments
    /// * `stake_pubkey` - The public key of the stake account
    /// * `unix_timestamp` - Optional new Unix timestamp for the lockup
    /// * `epoch` - Optional new epoch for the lockup
    /// * `custodian` - Optional new custodian public key (must sign if provided)
    /// * `custodian_pubkey` - The public key of the current custodian
    ///
    /// # Returns
    /// An instruction that needs to be executed with `process_transaction`
    pub fn set_lockup_checked(
        &mut self,
        stake_pubkey: &Pubkey,
        unix_timestamp: Option<i64>,
        epoch: Option<u64>,
        custodian: Option<Pubkey>,
        custodian_pubkey: &Pubkey,
    ) -> Instruction {
        let lockup_args = LockupArgs {
            unix_timestamp,
            epoch,
            custodian,
        };
        solana_stake_interface::instruction::set_lockup_checked(
            stake_pubkey,
            &lockup_args,
            custodian_pubkey,
        )
    }

    /// Creates an instruction to get the minimum delegation amount
    ///
    /// Generates an instruction that queries the minimum amount of lamports required
    /// for a stake account delegation.
    ///
    /// # Returns
    /// An instruction that needs to be executed with `process_transaction`
    pub fn get_minimum_delegation(&mut self) -> Instruction {
        solana_stake_interface::instruction::get_minimum_delegation()
    }

    /// Creates an instruction to deactivate a delinquent validator's stake
    ///
    /// Generates an instruction to deactivate stake delegated to a delinquent validator
    /// by providing a reference vote account that is not delinquent.
    ///
    /// # Arguments
    /// * `stake_pubkey` - The public key of the stake account to deactivate
    /// * `delinquent_vote_account` - The public key of the delinquent vote account
    /// * `reference_vote_account` - The public key of a non-delinquent vote account for comparison
    ///
    /// # Returns
    /// An instruction that needs to be executed with `process_transaction`
    pub fn deactivate_delinquent_stake(
        &mut self,
        stake_pubkey: &Pubkey,
        delinquent_vote_account: &Pubkey,
        reference_vote_account: &Pubkey,
    ) -> Instruction {
        solana_stake_interface::instruction::deactivate_delinquent_stake(
            stake_pubkey,
            delinquent_vote_account,
            reference_vote_account,
        )
    }

    /// Creates an instruction to move stake between accounts
    ///
    /// Generates an instruction to move a specified amount of active stake from one
    /// stake account to another. Both accounts must be delegated to the same vote account.
    ///
    /// # Arguments
    /// * `source_stake_pubkey` - The public key of the source stake account
    /// * `destination_stake_pubkey` - The public key of the destination stake account
    /// * `authorized_pubkey` - The public key of the stake authority
    /// * `lamports` - The number of lamports worth of stake to move
    ///
    /// # Returns
    /// An instruction that needs to be executed with `process_transaction`
    pub fn move_stake(
        &mut self,
        source_stake_pubkey: &Pubkey,
        destination_stake_pubkey: &Pubkey,
        authorized_pubkey: &Pubkey,
        lamports: u64,
    ) -> Instruction {
        solana_stake_interface::instruction::move_stake(
            source_stake_pubkey,
            destination_stake_pubkey,
            authorized_pubkey,
            lamports,
        )
    }

    /// Creates an instruction to move lamports between stake accounts
    ///
    /// Generates an instruction to move excess lamports (not part of active stake)
    /// from one stake account to another.
    ///
    /// # Arguments
    /// * `source_stake_pubkey` - The public key of the source stake account
    /// * `destination_stake_pubkey` - The public key of the destination stake account
    /// * `authorized_pubkey` - The public key of the stake authority
    /// * `lamports` - The number of lamports to move
    ///
    /// # Returns
    /// An instruction that needs to be executed with `process_transaction`
    pub fn move_lamports(
        &mut self,
        source_stake_pubkey: &Pubkey,
        destination_stake_pubkey: &Pubkey,
        authorized_pubkey: &Pubkey,
        lamports: u64,
    ) -> Instruction {
        solana_stake_interface::instruction::move_lamports(
            source_stake_pubkey,
            destination_stake_pubkey,
            authorized_pubkey,
            lamports,
        )
    }

    /// Retrieves and deserializes a stake account state
    ///
    /// Fetches the account data for a stake account and deserializes it into a `StakeStateV2` struct.
    ///
    /// # Arguments
    /// * `stake_pubkey` - The public key of the stake account to retrieve
    ///
    /// # Returns
    /// An `Option<StakeStateV2>` containing the deserialized stake state, or `None` if the account
    /// does not exist or deserialization fails
    pub fn get_stake_account(&self, stake_pubkey: &Pubkey) -> Option<StakeStateV2> {
        let account = self.get_account(stake_pubkey);
        let mut buf = account.data();
        StakeStateV2::deserialize(&mut buf).ok()
    }
}
