// COVERAGE CONSTANTS
pub const COVERAGE_IGNORE_REGEX: &str = "trident-tests|trident/crates";
pub const EXTENSION_NOTIFICATION_FILE: &str = "coverage-extension-notification.json";

pub const PROFRAW_FILENAME: &str = "fuzz-cov-%p-%30m.profraw";
pub const JSON_REPORT_FILENAME: &str = "coverage-report.json";
pub const HTML_REPORT_DIRNAME: &str = "coverage-report";
pub const COVERAGE_RUSTFLAGS: &str = "-C instrument-coverage -C link-arg=-fuse-ld=lld -C link-arg=-lprofiler";
