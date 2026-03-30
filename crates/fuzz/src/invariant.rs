/// Marker type for intentional invariant violations.
/// Used to distinguish user-defined invariant failures from unexpected panics.
#[derive(Debug)]
pub struct InvariantViolation(pub String);

/// Checks a condition and panics with `InvariantViolation` if false.
///
/// Use this macro to mark assertions as intentional invariant checks.
/// When an invariant fails, it will be counted and collected separately
/// from unexpected panics (bugs in fuzz test code).
///
/// # Examples
///
/// ```ignore
/// // Simple condition check
/// invariant!(balance_after == balance_before - amount);
/// invariant!(account.is_initialized);
/// invariant!(owner != Pubkey::default());
///
/// // With custom message
/// invariant!(balance > 0, "Balance must be positive");
/// invariant!(a == b, "Expected {} but got {}", a, b);
/// ```
#[macro_export]
macro_rules! invariant {
    ($cond:expr) => {
        if !$cond {
            std::panic::panic_any($crate::invariant::InvariantViolation(
                format!("invariant violation: {}", stringify!($cond))
            ));
        }
    };
    ($cond:expr, $($msg:tt)*) => {
        if !$cond {
            std::panic::panic_any($crate::invariant::InvariantViolation(format!($($msg)*)));
        }
    };
}

/// Checks if two expressions are equal and panics with `InvariantViolation` if not.
///
/// Use this macro to check if two expressions are equal.
/// When the expressions are not equal, it will be counted and collected separately
/// from unexpected panics (bugs in fuzz test code).
///
/// # Examples
///
/// ```ignore
/// // Simple condition check
/// invariant_eq!(balance_after, balance_before - amount);
/// invariant_eq!(account.is_initialized, true, "Account is not initialized");
/// ```
#[macro_export]
macro_rules! invariant_eq {
    ($a:expr, $b:expr) => {
        invariant!($a == $b);
    };
    ($a:expr, $b:expr, $($msg:tt)*) => {
        invariant!($a == $b, $($msg)*);
    };
}
