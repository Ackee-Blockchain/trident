use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct Cfg {
    pub cfg_identifier: String,
    pub val: bool,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Fuzz {
    pub rust_flags: Vec<Cfg>,
}
#[derive(Default, Debug, Deserialize, Clone)]
pub struct _Fuzz {
    #[serde(default)]
    pub allow_duplicate_txs: Option<bool>,
    #[serde(default)]
    pub fuzzing_with_stats: Option<bool>,
}
impl From<_Fuzz> for Fuzz {
    fn from(_f: _Fuzz) -> Self {
        let mut _self = Self { rust_flags: vec![] };

        // allow_duplicate_txs
        let allow_duplicate_txs = _f.allow_duplicate_txs.unwrap_or(false);

        _self.rust_flags.push(Cfg {
            cfg_identifier: "allow_duplicate_txs".to_string(),
            val: allow_duplicate_txs,
        });

        // fuzzing_with_stats
        let fuzzing_with_stats = _f.fuzzing_with_stats.unwrap_or(false);

        _self.rust_flags.push(Cfg {
            cfg_identifier: "fuzzing_with_stats".to_string(),
            val: fuzzing_with_stats,
        });

        _self
    }
}
