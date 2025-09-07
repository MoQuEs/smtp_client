use crate::database::{Database, DatabaseTrait, KeyValueDatabase, Section};
use crate::migration::{Migration, MigrationVersion, MIGRATIONS};
use crate::response::AnyResult;
use std::sync::Mutex;
use tauri::{Manager, State, Wry};

pub type AppHandle = tauri::AppHandle<Wry>;

pub struct AppState {
    pub db: Mutex<Option<Database>>,
}

pub trait ServiceAccess {
    fn setup(&self, app: &tauri::App<Wry>) {
        log::trace!("setup");
        log::info!("start setup");

        self.init();
        self.init_db();
        #[cfg(debug_assertions)]
        self.clear_db();
        self.migrate_db();

        log::trace!("setup end");
    }

    fn init(&self);

    fn init_db(&self);

    fn clear_db(&self);

    fn migrate_db(&self);

    fn run_migration<'a>(
        &self,
        current_version: &'a Migration,
        migration: &'a Migration,
    ) -> AnyResult<&'a Migration>;

    fn db<F, TResult>(&self, operation: F) -> TResult
    where
        F: FnOnce(&Database) -> TResult;

    fn db_mut<F, TResult>(&self, operation: F) -> TResult
    where
        F: FnOnce(&mut Database) -> TResult;
}

impl ServiceAccess for AppHandle {
    fn init(&self) {
        log::trace!("init");
        log::trace!("init end")
    }

    fn init_db(&self) {
        log::trace!("init_db");

        let db = Database::new()
            .inspect_err(|e| log::error!("Database initialize failed '{e:?}'"))
            .unwrap();

        let app_state: State<AppState> = self.state();
        *app_state
            .db
            .lock()
            .inspect_err(|e| log::error!("Lock database for data failed '{e:?}'"))
            .unwrap() = Some(db);

        log::trace!("init_db end");
    }

    fn clear_db(&self) {
        log::trace!("clear_db");

        self.db_mut(|db| {
            for section in Section::get_all() {
                db.clear(section).expect("Clear database section failed");
            }
        });

        log::trace!("clear_db end");
    }

    fn migrate_db(&self) {
        log::trace!("migrate");

        let current_version = self
            .db(|db| db.get_value("version"))
            .inspect_err(|e| log::error!("Get database version failed '{e:?}'"))
            .unwrap()
            .unwrap_or(MigrationVersion::default());

        log::debug!("migrate from version: {:?}", current_version);

        let mut current_version = MIGRATIONS
            .iter()
            .find(|fv| fv.version == current_version)
            .expect("Current migration version not found in migrations list");

        for migration in MIGRATIONS {
            current_version = self
                .run_migration(current_version, migration)
                .inspect_err(|e| log::error!("Run migration failed '{e:?}'"))
                .unwrap();
        }

        log::trace!("migrate end")
    }

    fn run_migration<'a>(
        &self,
        current: &'a Migration,
        migration: &'a Migration,
    ) -> AnyResult<&'a Migration> {
        log::trace!("run_migration");

        let mut err = None;

        log::info!("Current version: {:?}", current.version);
        if migration.idx <= current.idx {
            log::info!("Skip migration: {:?}", migration.version);
            return Ok(current);
        }

        log::info!("Run migration: {:?}", migration.idx);
        if let Err(e) = (migration.run)(self) {
            log::error!("Migration to {:?} failed: {:?}", migration.version, e);
            err = Some(e);

            log::warn!("Undo migration: {:?}", migration.version);
            if let Err(e) = (migration.undo)(self) {
                log::error!("Undo migration to {:?} failed: {:?}", migration.version, e);
            }
        }

        self.db(|db| db.set_value("version", &migration.version))
            .inspect_err(|e| log::error!("Set database version failed '{e:?}'"))?;

        if let Some(e) = err {
            return Err(e);
        }

        log::info!("Migration done: {:?}", migration.version);

        Ok(migration)
    }

    fn db<F, TResult>(&self, operation: F) -> TResult
    where
        F: FnOnce(&Database) -> TResult,
    {
        log::trace!("db");

        let app_state: State<AppState> = self.state();
        let db_connection_guard = app_state
            .db
            .lock()
            .inspect_err(|e| log::error!("Lock database failed '{e:?}'"))
            .expect("Lock database failed");

        let db = db_connection_guard
            .as_ref()
            .expect("Get database connection failed");

        log::info!("run operation");

        operation(db)
    }

    fn db_mut<F, TResult>(&self, operation: F) -> TResult
    where
        F: FnOnce(&mut Database) -> TResult,
    {
        log::trace!("db_mut");

        let app_state: State<AppState> = self.state();
        let mut db_connection_guard = app_state
            .db
            .lock()
            .inspect_err(|e| log::error!("Lock database failed '{e:?}'"))
            .expect("Lock database failed");

        let db = db_connection_guard
            .as_mut()
            .expect("Get database connection failed");

        log::info!("run operation");

        operation(db)
    }
}
