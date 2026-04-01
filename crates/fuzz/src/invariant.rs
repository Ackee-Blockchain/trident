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

/// Checks that two expressions are equal.
///
/// On failure, displays the actual values of both sides.
///
/// # Examples
///
/// ```ignore
/// invariant_eq!(balance_after, balance_before - amount);
/// invariant_eq!(account.owner, program_id, "wrong owner");
/// ```
#[macro_export]
macro_rules! invariant_eq {
    ($a:expr, $b:expr) => {
        {
            let left = &$a;
            let right = &$b;
            if left != right {
                std::panic::panic_any($crate::invariant::InvariantViolation(
                    format!(
                        "invariant violation: `{}` == `{}`, got: {:?} vs {:?}",
                        stringify!($a), stringify!($b), left, right
                    )
                ));
            }
        }
    };
    ($a:expr, $b:expr, $($msg:tt)*) => {
        {
            let left = &$a;
            let right = &$b;
            if left != right {
                std::panic::panic_any($crate::invariant::InvariantViolation(
                    format!("{} (got: {:?} vs {:?})", format!($($msg)*), left, right)
                ));
            }
        }
    };
}

/// Checks that two expressions are not equal.
///
/// On failure, displays the value that both sides unexpectedly share.
///
/// # Examples
///
/// ```ignore
/// invariant_ne!(owner, Pubkey::default());
/// invariant_ne!(balance_after, balance_before, "balance should have changed");
/// ```
#[macro_export]
macro_rules! invariant_ne {
    ($a:expr, $b:expr) => {
        {
            let left = &$a;
            let right = &$b;
            if left == right {
                std::panic::panic_any($crate::invariant::InvariantViolation(
                    format!(
                        "invariant violation: `{}` != `{}`, but both are: {:?}",
                        stringify!($a), stringify!($b), left
                    )
                ));
            }
        }
    };
    ($a:expr, $b:expr, $($msg:tt)*) => {
        {
            let left = &$a;
            let right = &$b;
            if left == right {
                std::panic::panic_any($crate::invariant::InvariantViolation(
                    format!("{} (both are: {:?})", format!($($msg)*), left)
                ));
            }
        }
    };
}

/// Checks that the first expression is strictly greater than the second.
///
/// # Examples
///
/// ```ignore
/// invariant_gt!(balance, 0);
/// invariant_gt!(supply_after, supply_before, "supply should increase after mint");
/// ```
#[macro_export]
macro_rules! invariant_gt {
    ($a:expr, $b:expr) => {
        {
            let left = &$a;
            let right = &$b;
            if !(left > right) {
                std::panic::panic_any($crate::invariant::InvariantViolation(
                    format!(
                        "invariant violation: `{}` > `{}`, got: {:?} vs {:?}",
                        stringify!($a), stringify!($b), left, right
                    )
                ));
            }
        }
    };
    ($a:expr, $b:expr, $($msg:tt)*) => {
        {
            let left = &$a;
            let right = &$b;
            if !(left > right) {
                std::panic::panic_any($crate::invariant::InvariantViolation(
                    format!("{} (got: {:?} vs {:?})", format!($($msg)*), left, right)
                ));
            }
        }
    };
}

/// Checks that the first expression is greater than or equal to the second.
///
/// # Examples
///
/// ```ignore
/// invariant_gte!(balance, minimum_balance);
/// invariant_gte!(lamports, rent_exempt, "account not rent-exempt");
/// ```
#[macro_export]
macro_rules! invariant_gte {
    ($a:expr, $b:expr) => {
        {
            let left = &$a;
            let right = &$b;
            if !(left >= right) {
                std::panic::panic_any($crate::invariant::InvariantViolation(
                    format!(
                        "invariant violation: `{}` >= `{}`, got: {:?} vs {:?}",
                        stringify!($a), stringify!($b), left, right
                    )
                ));
            }
        }
    };
    ($a:expr, $b:expr, $($msg:tt)*) => {
        {
            let left = &$a;
            let right = &$b;
            if !(left >= right) {
                std::panic::panic_any($crate::invariant::InvariantViolation(
                    format!("{} (got: {:?} vs {:?})", format!($($msg)*), left, right)
                ));
            }
        }
    };
}

/// Checks that the first expression is strictly less than the second.
///
/// # Examples
///
/// ```ignore
/// invariant_lt!(balance_after, balance_before);
/// invariant_lt!(fee, max_fee, "fee exceeds maximum");
/// ```
#[macro_export]
macro_rules! invariant_lt {
    ($a:expr, $b:expr) => {
        {
            let left = &$a;
            let right = &$b;
            if !(left < right) {
                std::panic::panic_any($crate::invariant::InvariantViolation(
                    format!(
                        "invariant violation: `{}` < `{}`, got: {:?} vs {:?}",
                        stringify!($a), stringify!($b), left, right
                    )
                ));
            }
        }
    };
    ($a:expr, $b:expr, $($msg:tt)*) => {
        {
            let left = &$a;
            let right = &$b;
            if !(left < right) {
                std::panic::panic_any($crate::invariant::InvariantViolation(
                    format!("{} (got: {:?} vs {:?})", format!($($msg)*), left, right)
                ));
            }
        }
    };
}

/// Checks that the first expression is less than or equal to the second.
///
/// # Examples
///
/// ```ignore
/// invariant_lte!(withdrawal, balance);
/// invariant_lte!(total_supply, max_supply, "supply overflow");
/// ```
#[macro_export]
macro_rules! invariant_lte {
    ($a:expr, $b:expr) => {
        {
            let left = &$a;
            let right = &$b;
            if !(left <= right) {
                std::panic::panic_any($crate::invariant::InvariantViolation(
                    format!(
                        "invariant violation: `{}` <= `{}`, got: {:?} vs {:?}",
                        stringify!($a), stringify!($b), left, right
                    )
                ));
            }
        }
    };
    ($a:expr, $b:expr, $($msg:tt)*) => {
        {
            let left = &$a;
            let right = &$b;
            if !(left <= right) {
                std::panic::panic_any($crate::invariant::InvariantViolation(
                    format!("{} (got: {:?} vs {:?})", format!($($msg)*), left, right)
                ));
            }
        }
    };
}
