use secrecy::{ExposeSecret, Secret};

#[derive(serde::Deserialize)]
pub struct Settings {
    pub database: DatabaseSettings,
    pub application: ApplicationSettings,
}

#[derive(serde::Deserialize)]
pub struct ApplicationSettings {
    pub port: u16,
    pub host: String,
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
            self.username,
            self.password.expose_secret(),
            self.host,
            self.port,
            self.database_name
        ))
    }

    pub fn get_connection_string_without_db(&self) -> Secret<String> {
        Secret::new(format!(
            "postgresql://{}:{}@{}:{}",
            self.username,
            self.password.expose_secret(),
            self.host,
            self.port
        ))
    }
}

pub enum Environment {
    Local,
    Production,
}

impl Environment {
    pub fn as_str(&self) -> &'static str {
        match self {
            Environment::Local => "local",
            Environment::Production => "production",
        }
    }
}

impl TryFrom<String> for Environment {
    type Error = String;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        match s.to_lowercase().as_str() {
            "local" => Ok(Self::Local),
            "production" => Ok(Self::Production),
            other => Err(format!(
                "{} is not a supported environment. Use either `local` or `production`.",
                other
            )),
        }
    }
}

pub fn get_configuration() -> Result<Settings, config::ConfigError> {
    // We are making our configuration hierarchical by doing the following
    // 1. First we read the base config file, this is environment variable that will be shared all across build environment
    // 2. Then we load the environment specific configuration, specifiying fields that will be different across build environment
    // 3. To determine which build environment we are doing, we use 'APP_ENVIRONMENT' (local or production)

    let mut settings = config::Config::default();
    let base_path = std::env::current_dir().expect("Unable to determine the current directory");
    let configuration_directory = base_path.join("configuration");

    // Detect the running environment
    // Default to `local` if unspecified
    let environment: Environment = std::env::var("APP_ENVIRONMENT")
        .unwrap_or_else(|_| "local".into())
        .try_into()
        .expect("Failed to parse APP_ENVIRONMENT");

    //Read config file
    settings.merge(config::File::from(configuration_directory.join("base")).required(true))?;
    settings.merge(
        config::File::from(configuration_directory.join(environment.as_str())).required(true),
    )?;

    settings.try_into()
}
