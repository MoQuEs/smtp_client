use crate::migration_version;
use std::fmt::{Display, Formatter};

mod migration_version;

mod v0_0_0;
mod v0_4_0;

migration_version!((v0_0_0, V0_0_0, "0.0.0"), (v0_4_0, V0_4_0, "0.4.0"));
