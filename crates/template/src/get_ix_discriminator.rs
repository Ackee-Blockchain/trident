use sha2::{Digest, Sha256};

use convert_case::{Case, Casing};
use trident_idl_spec::IdlInstruction;

pub const SIGHASH_GLOBAL_NAMESPACE: &str = "global";

pub(crate) fn process_discriminator(instruction: &IdlInstruction) -> Vec<u8> {
    // if discriminator is not provided, generate it
    if instruction.discriminator.is_empty() {
        let ix_name_snake_case = instruction.name.to_case(Case::Snake);
        gen_discriminator(SIGHASH_GLOBAL_NAMESPACE, &ix_name_snake_case).to_vec()
    } else {
        // if discriminator is provided, use it
        instruction.discriminator.clone()
    }
}

fn gen_discriminator(namespace: &str, name: &str) -> [u8; 8] {
    let preimage = format!("{namespace}:{name}");

    let mut hasher = Sha256::new();
    hasher.update(preimage);

    let mut sighash = [0u8; 8];
    sighash.copy_from_slice(&hasher.finalize().as_slice()[..8]);
    sighash
}
