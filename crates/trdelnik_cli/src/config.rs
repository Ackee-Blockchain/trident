use clap::Args;

#[derive(Args)]
pub struct ConfigOverride {
    /// Cluster override
    #[clap(global = true, long = "custom.cluster")]
    pub cluster: Option<String>,
    /// Wallet override
    #[clap(global = true, long = "custom.wallet")]
    pub wallet: Option<String>,
}
