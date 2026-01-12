#[derive(Debug, Clone)]
pub struct AppConfig {
    pub default_output_dir: String,
    pub seed: u64,
}

impl Default for AppConfig {
    fn default() -> Self {
        AppConfig {
            default_output_dir: "output".to_string(),
            seed: 42,
        }
    }
}
