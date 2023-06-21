#[macro_export]
macro_rules! migration_version {
    ($( ($module_version:ident, $version:ident, $version_str:expr) ),*) => {
        use $crate::state::AppHandle;

        #[derive(serde::Deserialize, serde::Serialize, Default, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
        pub enum MigrationVersion {
            #[default]
            $( $version, )*
        }

        impl From<&str> for MigrationVersion {
            fn from(version: &str) -> Self {
                match version {
                    $( $version_str => Self::$version, )*
                    _ => panic!("Unknown migration version: {version}"),
                }
            }
        }

        impl Display for MigrationVersion {
            fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}", self)
            }
        }

        impl MigrationVersion {
            pub fn run(&self, app_handle: &AppHandle) -> Result<MigrationVersion, MigrationVersion> {
                log::trace!(target: "backend::migration::MigrationVersion::run", "run");

                let mut current_version = *self;

                $(
                    log::debug!(target: "backend::migration::MigrationVersion::run", "Current version: {}", &current_version);
                    if current_version <= MigrationVersion::$version {
                        log::info!(target: "backend::migration::MigrationVersion::run", "Skip migration: {}", MigrationVersion::$version);
                    } else {
                        log::info!(target: "backend::migration::MigrationVersion::run", "Run migration: {}", MigrationVersion::$version);
                        current_version = $crate::migration::$module_version::run(app_handle).map_err(|_| current_version)?;
                        log::info!(target: "backend::migration::MigrationVersion::run", "Migration done: {}", MigrationVersion::$version);
                    }
                )*

                log::trace!(target: "backend::migration::MigrationVersion::run", "end run");
                Ok(current_version)
            }
        }
    }
}
