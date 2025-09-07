use crate::migration_version;

mod migration_version;

pub mod versions;

migration_version!(
    (1, v0_0_0, V0_0_0, "0.0.0"),
    (2, v0_4_0, V0_4_0, "0.4.0"),
    (3, v0_5_0, V0_5_0, "0.5.0")
);
