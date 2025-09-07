#[macro_export]
macro_rules! migration_version {
    ($( ($version_idx:expr, $module_version:ident, $version:ident, $version_str:expr) ),*) => {
        use $crate::state::AppHandle;

        #[derive(serde::Deserialize, serde::Serialize, bincode::Encode, bincode::Decode, Default, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
        pub enum MigrationVersion {
            #[default]
            $( $version, )*
        }

        pub type MigrationCallback = fn(&AppHandle) -> $crate::response::AnyResult<MigrationVersion>;

        pub struct Migration {
            pub idx: usize,
            pub version: MigrationVersion,
            pub run: MigrationCallback,
            pub undo: MigrationCallback,
        }

        pub const MIGRATIONS: &[Migration] = &[
            $( Migration {
                idx: $version_idx,
                version: MigrationVersion::$version,
                run: versions::$module_version::run,
                undo: versions::$module_version::undo,
            }, )*
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

        impl MigrationVersion {
            pub fn as_str(&self) -> &str {
                match self {
                    $( Self::$version => $version_str, )*
                }
            }

            pub fn as_idx(&self) -> usize {
                match self {
                    $( Self::$version => $version_idx, )*
                }
            }
        }
    }
}
