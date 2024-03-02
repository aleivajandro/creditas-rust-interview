#[derive(serde::Deserialize)]
pub struct ApplicationConfig {
    pub language: String,
    pub country: String,
}

pub fn get_config(filename: &str) -> Result<ApplicationConfig, config::ConfigError> {
    let mut settings = config::Config::default();
    settings.merge(config::File::with_name(filename))?;
    settings.try_into()
}