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
