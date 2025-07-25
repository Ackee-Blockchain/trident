// Dashboard-related types
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct DashboardConfig {
    pub title: String,
    pub show_success_rate: bool,
    pub show_error_details: bool,
    pub show_panic_analysis: bool,
    pub show_invariant_failures: bool,
}

impl Default for DashboardConfig {
    fn default() -> Self {
        Self {
            title: "Trident Fuzzing Dashboard".to_string(),
            show_success_rate: true,
            show_error_details: true,
            show_panic_analysis: true,
            show_invariant_failures: true,
        }
    }
}

impl DashboardConfig {
    pub fn new_with_title(title: String) -> Self {
        Self {
            title,
            ..Default::default()
        }
    }
    pub fn with_default_title() -> Self {
        Self {
            title: "Trident Fuzzing Dashboard".to_string(),
            ..Default::default()
        }
    }
}
