mod get_ix_discriminator;
mod get_program_id;
mod template_custom_types;
// mod instruction_variants;

use get_ix_discriminator::*;
use get_program_id::*;
// use instruction_variants::*;

pub mod template;

pub use template::Template;
mod idl_type_to_syn_type;
mod template_fuzz_transactions;
pub mod template_getters;
mod template_instruction;
mod template_transaction;
mod test_fuzz;
use idl_type_to_syn_type::idl_type_to_syn_type;
