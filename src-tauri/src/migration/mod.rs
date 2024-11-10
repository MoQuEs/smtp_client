use crate::migration_version;

mod migration_version;
pub mod versions;

migration_version!((v0_0_0, V0_0_0, "0.0.0"), (v0_4_0, V0_4_0, "0.4.0"));
