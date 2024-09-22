use serde::Deserialize;

#[derive(Debug, Deserialize, Clone, Default)]
pub struct Fuzz {
    pub fuzzing_with_stats: bool,
    pub allow_duplicate_txs: bool,
}

#[derive(Default, Debug, Deserialize, Clone)]
pub struct _Fuzz {
    #[serde(default)]
    pub fuzzing_with_stats: Option<bool>,
    #[serde(default)]
    pub allow_duplicate_txs: Option<bool>,
}
impl From<_Fuzz> for Fuzz {
    fn from(_f: _Fuzz) -> Self {
        Self {
            fuzzing_with_stats: _f.fuzzing_with_stats.unwrap_or_default(),
            allow_duplicate_txs: _f.allow_duplicate_txs.unwrap_or_default(),
        }
    }
}

impl Fuzz {
    pub fn get_fuzzing_with_stats(&self) -> bool {
        self.fuzzing_with_stats
    }
    pub fn get_allow_duplicate_txs(&self) -> bool {
        self.allow_duplicate_txs
    }
}
