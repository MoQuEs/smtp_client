#[macro_export]
macro_rules! migration_version {
    ($( ($module_version:ident, $version:ident, $version_str:expr) ),*) => {
        use $crate::state::AppHandle;

        #[derive(serde::Deserialize, serde::Serialize, bincode::Encode, bincode::Decode, Default, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
        pub enum MigrationVersion {
            #[default]
            $( $version, )*
        }

        pub const MIGRATIONS: &[(
            MigrationVersion,
            fn(&AppHandle) -> $crate::response::AnyResult<MigrationVersion>,
        )] = &[
            $( (MigrationVersion::$version, versions::$module_version::run), )*
        ];

        impl From<&str> for MigrationVersion {
            fn from(version: &str) -> Self {
                match version {
                    $( $version_str => Self::$version, )*
                    _ => panic!("Unknown migration version: {version}"),
                }
            }
        }

        impl std::fmt::Display for MigrationVersion {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}", self)
            }
        }
    }
}
