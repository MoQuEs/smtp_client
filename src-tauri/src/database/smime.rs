use crate::database::{Database, DatabaseTrait, Section};
use crate::response::*;

pub trait SmimeDatabase {
    fn save_smime(&self, smime: &NamedSmime) -> AnyResult<()>;
    fn remove_smime(&self, smime: &NamedSmime) -> AnyResult<()>;
    fn get_smimes(&self) -> AnyResult<NamedSmimes>;
}

impl SmimeDatabase for Database {
    fn save_smime(&self, smime: &NamedSmime) -> AnyResult<()> {
        log::trace!("save_smime");
        log::debug!("smime: {:?}", smime);

        self.insert(Section::Smime, smime.name.as_str(), smime)
    }

    fn remove_smime(&self, smime: &NamedSmime) -> AnyResult<()> {
        log::trace!("remove_smime");
        log::debug!("smime: {:?}", smime);

        self.remove(Section::Smime, smime.name.as_str())
    }

    fn get_smimes(&self) -> AnyResult<NamedSmimes> {
        log::trace!("get_smimes");

        self.get_all(Section::Smime)
    }
}
