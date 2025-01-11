mod custom_types;
mod fuzz_accounts;
mod get_accounts;
mod get_data;
mod get_ix_discriminator;
mod get_program_id;
mod instruction_account;
mod instruction_inputs;
mod instruction_ixops;
mod instruction_variants;

use custom_types::*;
use fuzz_accounts::*;
use get_ix_discriminator::*;
use get_program_id::*;
use instruction_inputs::*;
use instruction_ixops::*;
use instruction_variants::*;

pub mod fuzz_instructions_generator;
pub mod test_fuzz_generator;
