use secrecy::{ExposeSecret, Secret};

#[derive(serde::Deserialize)]
pub struct Settings {
    pub database: DatabaseSettings,
    pub application_port: u16,
}

#[derive(serde::Deserialize)]
pub struct DatabaseSettings {
    pub username: String,
    pub password: Secret<String>,
    pub port: u16,
    pub host: String,
    pub database_name: String,
}

impl DatabaseSettings {
    pub fn get_connection_string(&self) -> Secret<String> {
        Secret::new(format!(
            "postgresql://{}:{}@{}:{}/{}",
            self.username, self.password.expose_secret(), self.host, self.port, self.database_name
        ))
    }

    pub fn get_connection_string_without_db(&self) -> Secret<String> {
        Secret::new(format!(
            "postgresql://{}:{}@{}:{}",
            self.username, self.password.expose_secret(), self.host, self.port
        ))
    }
}

pub fn get_configuration() -> Result<Settings, config::ConfigError> {
    //Initialize configuration reader
    let mut settings = config::Config::default();

    //Add configuration file
    settings.merge(config::File::with_name("configuration"))?;

    //Try to convert the configuration into our Settings type
    settings.try_into()
}
