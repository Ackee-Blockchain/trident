use std::fmt;
use std::str::FromStr;

/// Specifies which type of failures should cause a non-zero exit code.
///
/// - `All`: Exit non-zero on any policy failure (program panics or custom invariant failures)
/// - `Invariants`: Exit non-zero only on custom invariant failures in fuzz tests
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ExitCodeMode {
    /// Exit non-zero on any policy failure (default behavior when exit code is enabled)
    #[default]
    All,
    /// Exit non-zero only on custom invariant failures in fuzz tests
    Invariants,
}

impl ExitCodeMode {
    /// Returns the environment variable value for this mode
    pub fn to_env_value(&self) -> &'static str {
        match self {
            ExitCodeMode::All => "all",
            ExitCodeMode::Invariants => "invariants",
        }
    }

    /// Check if this mode should trigger exit code for invariant failures
    pub fn triggers_on_invariants(&self) -> bool {
        matches!(self, ExitCodeMode::All | ExitCodeMode::Invariants)
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
            _ => Err(format!(
                "Invalid exit code mode '{}'. Valid values are: all, invariants",
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
        assert_eq!(ExitCodeMode::from_str("ALL").unwrap(), ExitCodeMode::All);
        assert!(ExitCodeMode::from_str("invalid").is_err());
    }

    #[test]
    fn test_triggers() {
        assert!(ExitCodeMode::All.triggers_on_invariants());
        assert!(ExitCodeMode::Invariants.triggers_on_invariants());
    }
}
