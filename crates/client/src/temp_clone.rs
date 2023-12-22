// @TODO remove once `Clone` is implemented for `Keypair`
// https://docs.rs/solana-sdk/latest/solana_sdk/signer/keypair/struct.Keypair.html

// probably will not be added into the Keypair
// https://docs.rs/solana-sdk/latest/solana_sdk/signer/keypair/struct.Keypair.html#method.insecure_clone

/// The `TempClone` trait is used as a workaround
/// for making non-cloneable foreign types cloneable.
pub trait TempClone {
    fn clone(&self) -> Self;
}

impl TempClone for solana_sdk::signer::keypair::Keypair {
    fn clone(&self) -> Self {
        Self::from_bytes(&self.to_bytes()).unwrap()
    }
}
