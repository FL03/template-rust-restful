/*
   Appellation: settings
   Context:
   Creator: FL03 <jo3mccain@icloud.com>
   Description:
       ... Summary ...
*/
pub use components::*;
pub use utils::*;

#[derive(Clone, Debug, Hash, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct Configuration {
    pub application: Application,
    pub database: Database,
    pub logger: Logger,
    pub server: Server
}

impl Configuration {
    pub fn constructor() -> Result<ConfigBuilderDS, config::ConfigError> {
        let mut builder = config::Config::builder()
            .set_default("application.mode", "development")?
            .set_default("application.name", "maximus")?
            .set_default("database.name", "postgres")?
            .set_default("database.uri", "postgres://postgres:example@localhost:5432/postgres")?
            .set_default("logger.level", "info")?;

        builder = builder.add_source(collect_config_files("**/*.config.*"));
        builder = builder.add_source(config::Environment::default().separator("__"));
        Ok(
            builder
        )
    }
    pub fn new() -> Result<Self, config::ConfigError> {
        Self::constructor().ok().unwrap().build()?.try_deserialize()
    }
}

impl std::fmt::Display for Configuration {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Configuration(application={}, database={}, logger={})",
            self.application, self.database, self.logger
        )
    }
}

mod components {
    #[derive(Clone, Debug, Hash, PartialEq, serde::Deserialize, serde::Serialize)]
    pub struct Application {
        pub mode: String,
        pub name: String,
    }

    impl std::fmt::Display for Application {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "Application(mode={}, name={})", self.mode, self.name)
        }
    }

    #[derive(Clone, Debug, Hash, PartialEq, serde::Deserialize, serde::Serialize)]
    pub struct Database {
        pub name: String,
        pub uri: String,
    }

    impl std::fmt::Display for Database {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "Database(name={}, uri={})", self.name, self.uri)
        }
    }

    #[derive(Clone, Debug, Hash, PartialEq, serde::Deserialize, serde::Serialize)]
    pub struct Server {
        pub port: u16,
    }

    impl std::fmt::Display for Server {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "Server(port={})", self.port)
        }
    }


    #[derive(Clone, Debug, Hash, PartialEq, serde::Deserialize, serde::Serialize)]
    pub struct Logger {
        pub level: String,
    }

    impl Logger {
        pub fn setup(configuration: &crate::Configuration) {
            if std::env::var_os("RUST_LOG").is_none() {
                let app_name = configuration.application.name.as_str();
                let level = configuration.logger.level.as_str();
                let env = format!("api={},tower_http={}", app_name, level);

                std::env::set_var("RUST_LOG", env);
            }

            tracing_subscriber::fmt::init();
        }
    }

    impl std::fmt::Display for Logger {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "Logger(level={})", self.level)
        }
    }
}

mod utils {

    pub type ConfigBuilderDS = config::ConfigBuilder<config::builder::DefaultState>;
    pub type ConfigFromFileVec = Vec<config::File<config::FileSourceFile, config::FileFormat>>;

    pub fn collect_config_files(pattern: &str) -> ConfigFromFileVec {
        glob::glob(pattern)
            .unwrap()
            .map(|path| config::File::from(path.unwrap()).required(false))
            .collect::<Vec<_>>()
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        let f = |x: usize| x.pow(x.try_into().unwrap());
        assert_eq!(f(2), 4)
    }
}
