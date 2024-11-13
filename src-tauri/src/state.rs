use crate::database::Database;
use crate::migration::{MigrationVersion, MIGRATIONS};
use crate::response::AnyResult;
use rust_utils::log::LogUnwrap;
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
        //self.migrate();

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

        let db = Database::new(app.config()).log_error_u(
            "backend::state::AppHandle::init_db",
            "Database initialize failed",
        );

        *app_state.db.lock().log_error_u(
            "backend::state::AppHandle::init_db",
            "Lock database for data",
        ) = Some(db);

        log::trace!("init_db end");
    }

    fn migrate(&self) {
        log::trace!("migrate");

        let mut current_version = self
            .db(|db| db.get_value("version"))
            .log_error_u(
                "backend::state::AppHandle::migrate",
                "Get database version failed",
            )
            .unwrap_or(MigrationVersion::default());

        log::debug!("migrate from version: {:?}", current_version);

        for (migration_version, callback) in MIGRATIONS {
            current_version = self
                .run_migration(current_version, migration_version, callback)
                .log_error_u("backend::state::AppHandle::migrate", "Run migration failed");
        }

        log::trace!("migrate end")
    }

    fn run_migration<F>(
        &self,
        current_version: MigrationVersion,
        migration_version: &MigrationVersion,
        callback: F,
    ) -> AnyResult<MigrationVersion>
    where
        F: FnOnce(&AppHandle) -> AnyResult<MigrationVersion>,
    {
        log::trace!("run_migration");

        log::info!("Current version: {:?}", current_version);
        if migration_version <= &current_version {
            log::info!("Skip migration: {:?}", migration_version);
        } else {
            log::info!("Run migration: {:?}", migration_version);
            if let Err(e) = callback(self) {
                log::error!("Migration to {:?} failed: {:?}", migration_version, e);
                return Err(e);
            }

            self.db(|db| db.set_value("version", migration_version))
                .log_error_u(
                    "backend::state::AppHandle::migrate",
                    "Set database version failed",
                );

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
            .log_error_u("backend::state::AppHandle::db", "Lock database");

        let db = db_connection_guard
            .as_ref()
            .log_error_u("backend::state::AppHandle::db", "Get database connection");

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
            .log_error_u("backend::state::AppHandle::db_mut", "Lock database");

        let db = db_connection_guard
            .as_mut()
            .log_error_u("backend::state::AppHandle::db", "Get database connection");

        log::info!("run operation");

        operation(db)
    }
}
