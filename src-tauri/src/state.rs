use crate::database::{Database, DatabaseTrait};
use crate::migration::{MigrationVersion, MIGRATIONS};
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

        self.init(app);
        self.init_db(app);
        self.migrate();

        log::trace!("setup end");
    }

    fn init(&self, app: &tauri::App<Wry>);

    fn init_db(&self, app: &tauri::App<Wry>);

    fn migrate(&self);

    fn run_migration<F>(
        &self,
        current_version: MigrationVersion,
        migration_version: &MigrationVersion,
        callback: F,
        undo: F,
    ) -> AnyResult<MigrationVersion>
    where
        F: FnOnce(&AppHandle) -> AnyResult<MigrationVersion>;

    fn db<F, TResult>(&self, operation: F) -> TResult
    where
        F: FnOnce(&Database) -> TResult;

    fn db_mut<F, TResult>(&self, operation: F) -> TResult
    where
        F: FnOnce(&mut Database) -> TResult;
}

impl ServiceAccess for AppHandle {
    fn init(&self, app: &tauri::App<Wry>) {
        log::trace!("init");
        log::trace!("init end")
    }

    fn init_db(&self, app: &tauri::App<Wry>) {
        log::trace!("init_db");

        let app_state: State<AppState> = app.state();

        let db = Database::new()
            .inspect_err(|e| log::error!("Database initialize failed '{:?}'"))
            .unwrap();

        *app_state
            .db
            .lock()
            .inspect_err(|e| log::error!("Lock database for data failed '{:?}'"))
            .unwrap() = Some(db);

        log::trace!("init_db end");
    }

    fn migrate(&self) {
        log::trace!("migrate");

        let mut current_version = self
            .db(|db| db.get_value("version"))
            .inspect_err(|e| log::error!("Get database version failed '{:?}'"))
            .unwrap()
            .unwrap_or(MigrationVersion::default());

        log::debug!("migrate from version: {:?}", current_version);

        for (migration_version, callback, undo) in MIGRATIONS {
            current_version = self
                .run_migration(current_version, migration_version, callback, undo)
                .inspect_err(|e| log::error!("Run migration failed '{:?}'"))
                .unwrap();
        }

        log::trace!("migrate end")
    }

    fn run_migration<F>(
        &self,
        current_version: MigrationVersion,
        migration_version: &MigrationVersion,
        callback: F,
        undo: F,
    ) -> AnyResult<MigrationVersion>
    where
        F: FnOnce(&AppHandle) -> AnyResult<MigrationVersion>,
    {
        log::trace!("run_migration");

        let mut err = None;

        log::info!("Current version: {:?}", current_version);
        if migration_version <= &current_version {
            log::info!("Skip migration: {:?}", migration_version);
        } else {
            log::info!("Run migration: {:?}", migration_version);
            if let Err(e) = callback(self) {
                log::error!("Migration to {:?} failed: {:?}", migration_version, e);
                err = Some(e);

                log::warn!("Undo migration: {:?}", migration_version);
                if let Err(e) = undo(self) {
                    log::error!("Undo migration to {:?} failed: {:?}", migration_version, e);
                }
            }

            self.db(|db| db.set_value("version", migration_version))
                .inspect_err(|e| log::error!("Set database version failed '{:?}'"))?;

            if let Some(e) = err {
                return Err(e);
            }

            log::info!("Migration done: {:?}", migration_version);
        }

        Ok(*migration_version)
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
            .inspect_err(|e| log::error!("Lock database failed '{:?}'"))?;

        let db = db_connection_guard
            .as_ref()
            .inspect_err(|e| log::error!("Get database connection failed '{:?}'"))?;

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
            .inspect_err(|e| log::error!("Lock database failed '{:?}'"))?;

        let db = db_connection_guard
            .as_mut()
            .inspect_err(|e| log::error!("Get database connection failed '{:?}'"))?;

        log::info!("run operation");

        operation(db)
    }
}
