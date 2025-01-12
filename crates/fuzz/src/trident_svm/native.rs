use solana_program_runtime::invoke_context::BuiltinFunctionWithContext;
use solana_sdk::{
    bpf_loader, bpf_loader_deprecated, bpf_loader_upgradeable, feature_set, pubkey::Pubkey,
};

pub(crate) struct BuiltinPrototype {
    pub _feature_id: Option<Pubkey>,
    pub program_id: Pubkey,
    pub name: &'static str,
    pub entrypoint: BuiltinFunctionWithContext,
}

pub(crate) static BUILTINS: &[BuiltinPrototype] = &[
    BuiltinPrototype {
        _feature_id: None,
        program_id: solana_system_program::id(),
        name: "system_program",
        entrypoint: solana_system_program::system_processor::Entrypoint::vm,
    },
    BuiltinPrototype {
        _feature_id: None,
        program_id: solana_vote_program::id(),
        name: "vote_program",
        entrypoint: solana_vote_program::vote_processor::Entrypoint::vm,
    },
    BuiltinPrototype {
        _feature_id: None,
        program_id: solana_stake_program::id(),
        name: "stake_program",
        entrypoint: solana_stake_program::stake_instruction::Entrypoint::vm,
    },
    BuiltinPrototype {
        _feature_id: None,
        program_id: solana_config_program::id(),
        name: "config_program",
        entrypoint: solana_config_program::config_processor::Entrypoint::vm,
    },
    BuiltinPrototype {
        _feature_id: None,
        program_id: bpf_loader_deprecated::id(),
        name: "solana_bpf_loader_deprecated_program",
        entrypoint: solana_bpf_loader_program::Entrypoint::vm,
    },
    BuiltinPrototype {
        _feature_id: None,
        program_id: bpf_loader::id(),
        name: "solana_bpf_loader_program",
        entrypoint: solana_bpf_loader_program::Entrypoint::vm,
    },
    BuiltinPrototype {
        _feature_id: None,
        program_id: bpf_loader_upgradeable::id(),
        name: "solana_bpf_loader_upgradeable_program",
        entrypoint: solana_bpf_loader_program::Entrypoint::vm,
    },
    BuiltinPrototype {
        _feature_id: None,
        program_id: solana_sdk::compute_budget::id(),
        name: "compute_budget_program",
        entrypoint: solana_compute_budget_program::Entrypoint::vm,
    },
    BuiltinPrototype {
        _feature_id: None,
        program_id: solana_sdk::address_lookup_table::program::id(),
        name: "address_lookup_table_program",
        entrypoint: solana_address_lookup_table_program::processor::Entrypoint::vm,
    },
    // BuiltinPrototype {
    //     _feature_id: Some(feature_set::zk_token_sdk_enabled::id()),
    //     program_id: solana_zk_token_sdk::zk_token_proof_program::id(),
    //     name: "zk_token_proof_program",
    //     entrypoint: solana_zk_token_proof_program::Entrypoint::vm,
    // },
    BuiltinPrototype {
        _feature_id: Some(feature_set::enable_program_runtime_v2_and_loader_v4::id()),
        program_id: solana_sdk::loader_v4::id(),
        name: "loader_v4",
        entrypoint: solana_loader_v4_program::Entrypoint::vm,
    },
];
