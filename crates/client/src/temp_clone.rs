use solana_sdk::signer::keypair::Keypair;
// @TODO remove once `Clone` is implemented for `Keypair`
// https://docs.rs/solana-sdk/latest/solana_sdk/signer/keypair/struct.Keypair.html

/// The `TempClone` trait is used as a workaround
/// for making non-cloneable foreign types cloneable.
pub trait TempClone {
    fn clone(&self) -> Self;
}

impl TempClone for Keypair {
    fn clone(&self) -> Self {
        Self::from_bytes(&self.to_bytes()).unwrap()
    }
}
