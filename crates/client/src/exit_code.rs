use std::fmt;
use std::str::FromStr;

/// Specifies which type of failures should cause a non-zero exit code.
///
/// - `All`: Exit non-zero on any failure (program panics or invariant failures)
/// - `Invariants`: Exit non-zero only on invariant/assert failures in fuzz tests
/// - `Panics`: Exit non-zero only on program panics (program failed to complete)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ExitCodeMode {
    /// Exit non-zero on any failure (default behavior when exit code is enabled)
    #[default]
    All,
    /// Exit non-zero only on invariant/assert failures in fuzz tests
    Invariants,
    /// Exit non-zero only on program panics (program failed to complete)
    Panics,
}

impl ExitCodeMode {
    /// Returns the environment variable value for this mode
    pub fn to_env_value(&self) -> &'static str {
        match self {
            ExitCodeMode::All => "all",
            ExitCodeMode::Invariants => "invariants",
            ExitCodeMode::Panics => "panics",
        }
    }

    /// Check if this mode should trigger exit code for invariant failures
    pub fn triggers_on_invariants(&self) -> bool {
        matches!(self, ExitCodeMode::All | ExitCodeMode::Invariants)
    }

    /// Check if this mode should trigger exit code for program panics
    pub fn triggers_on_panics(&self) -> bool {
        matches!(self, ExitCodeMode::All | ExitCodeMode::Panics)
    }
}

impl fmt::Display for ExitCodeMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_env_value())
    }
}

impl FromStr for ExitCodeMode {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "all" => Ok(ExitCodeMode::All),
            "invariants" => Ok(ExitCodeMode::Invariants),
            "panics" => Ok(ExitCodeMode::Panics),
            _ => Err(format!(
                "Invalid exit code mode '{}'. Valid values are: all, invariants, panics",
                s
            )),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_str() {
        assert_eq!(ExitCodeMode::from_str("all").unwrap(), ExitCodeMode::All);
        assert_eq!(
            ExitCodeMode::from_str("invariants").unwrap(),
            ExitCodeMode::Invariants
        );
        assert_eq!(
            ExitCodeMode::from_str("panics").unwrap(),
            ExitCodeMode::Panics
        );
        assert_eq!(ExitCodeMode::from_str("ALL").unwrap(), ExitCodeMode::All);
        assert!(ExitCodeMode::from_str("invalid").is_err());
    }

    #[test]
    fn test_triggers() {
        assert!(ExitCodeMode::All.triggers_on_invariants());
        assert!(ExitCodeMode::All.triggers_on_panics());

        assert!(ExitCodeMode::Invariants.triggers_on_invariants());
        assert!(!ExitCodeMode::Invariants.triggers_on_panics());

        assert!(!ExitCodeMode::Panics.triggers_on_invariants());
        assert!(ExitCodeMode::Panics.triggers_on_panics());
    }
}
