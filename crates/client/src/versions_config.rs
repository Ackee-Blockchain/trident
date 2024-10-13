use serde::Deserialize;

#[derive(Deserialize, Default)]
pub struct TridentVersionsConfig {
    pub trident_fuzz: String,
    pub trident_derive_accounts_snapshots: String,
    pub trident_client: String,
}
