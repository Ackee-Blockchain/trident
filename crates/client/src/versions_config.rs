use serde::Deserialize;

#[derive(Deserialize, Default)]
pub struct TridentVersionsConfig {
    pub trident_fuzz: String,
}
