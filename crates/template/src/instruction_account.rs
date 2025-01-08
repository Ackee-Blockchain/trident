use std::collections::{HashMap, HashSet};

use trident_idl_spec::IdlPda;

pub(crate) struct InstructionAccount {
    // account name
    pub(crate) account_name: String,
    // account kind for each instruction, if its None , we dont know the account type is
    pub(crate) kind: HashMap<String, InstructionAccountType>,
    // account type for fuzz accounts struct
    // we only care about keypair / pda
    pub(crate) fuzz_accounts_type: HashSet<FuzzAccountsType>,
}

#[derive(Default, PartialEq, Eq, Hash, Clone, Debug)]
pub(crate) enum InstructionAccountType {
    // if we cannot decide the account type
    #[default]
    Unknown,
    // writable | signer
    Keypair(bool, bool),
    // writable | signer (PDA cannot be signer, but keep it simple)
    Pda(IdlPda, bool, bool),
    // writable | signer
    Constant(String, bool, bool),
}

// Unfortunatelly, to simplify stuff we need to have this enum
#[derive(Default, PartialEq, Eq, Hash, Clone, Debug)]
pub(crate) enum FuzzAccountsType {
    // if we cannot decide the account type
    #[default]
    Unknown,
    Keypair,
    Pda,
    Constant,
}

impl InstructionAccount {
    pub(crate) fn new(account_name: String) -> Self {
        Self {
            account_name,
            kind: HashMap::new(),
            fuzz_accounts_type: HashSet::new(),
        }
    }
    pub(crate) fn insert(
        &mut self,
        instruction_name: String,
        account_type: InstructionAccountType,
    ) {
        self.fuzz_accounts_type
            .insert(FuzzAccountsType::from(&account_type));

        self.kind.insert(instruction_name, account_type);
    }
    pub(crate) fn get_fuzz_accounts_type(&self) -> FuzzAccountsType {
        if self.fuzz_accounts_type.is_empty() {
            FuzzAccountsType::Unknown
        } else if self.fuzz_accounts_type.len() == 1 {
            self.fuzz_accounts_type.iter().next().unwrap().clone()
        } else {
            FuzzAccountsType::Unknown
        }
    }
}

impl From<&InstructionAccountType> for FuzzAccountsType {
    fn from(account_type: &InstructionAccountType) -> Self {
        match account_type {
            InstructionAccountType::Keypair(_, _) => FuzzAccountsType::Keypair,
            InstructionAccountType::Pda(_, _, _) => FuzzAccountsType::Pda,
            InstructionAccountType::Constant(_, _, _) => FuzzAccountsType::Constant,
            InstructionAccountType::Unknown => FuzzAccountsType::Unknown,
        }
    }
}
