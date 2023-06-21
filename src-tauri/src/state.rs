use crate::database::Database;
use crate::migration::MigrationVersion;
use rust_utils::log::LogU;
use std::sync::Mutex;
use tauri::{Manager, State, Wry};

pub type AppHandle = tauri::AppHandle<Wry>;

pub struct AppState {
    pub db: Mutex<Option<Database>>,
}

pub trait ServiceAccess {
    fn setup(&self, app: &tauri::App<Wry>) {
        log::trace!(target: "backend::state::AppHandle::setup", "setup");
        log::info!(target: "backend::state::AppHandle::setup", "start setup");

        self.init(app);
        self.init_db(app);
        //self.migrate();

        log::trace!(target: "backend::state::AppHandle::setup", "setup end");
    }

    fn init(&self, app: &tauri::App<Wry>);

    fn init_db(&self, app: &tauri::App<Wry>);

    fn migrate(&self);

    fn db<F, TResult>(&self, operation: F) -> TResult
    where
        F: FnOnce(&Database) -> TResult;

    fn db_mut<F, TResult>(&self, operation: F) -> TResult
    where
        F: FnOnce(&mut Database) -> TResult;
}

impl ServiceAccess for AppHandle {
    fn init(&self, app: &tauri::App<Wry>) {
        log::trace!(target: "backend::state::AppHandle::init", "init");
        log::trace!(target: "backend::state::AppHandle::init", "init end")
    }

    fn init_db(&self, app: &tauri::App<Wry>) {
        log::trace!(target: "backend::state::AppHandle::init_db", "init_db");

        let app_state: State<AppState> = app.state();

        let db = Database::new(app.config().as_ref()).log_error_u(
            "backend::state::AppHandle::init_db",
            "Database initialize failed",
        );

        *app_state.db.lock().log_error_u(
            "backend::state::AppHandle::init_db",
            "Lock database for data",
        ) = Some(db);

        log::trace!(target: "backend::state::AppHandle::init_db", "init_db end");
    }

    fn migrate(&self) {
        log::trace!(target: "backend::state::AppHandle::migrate", "migrate");

        let version = self
            .db(|db| db.get_value("version"))
            .log_error_u(
                "backend::state::AppHandle::migrate",
                "Get database version failed",
            )
            .unwrap_or(MigrationVersion::default());

        log::debug!(target: "backend::state::AppHandle::migrate", "migrate from version: {:?}", version);

        let migrated_version = version.run(self).log_error_u(
            "backend::state::AppHandle::migrate",
            format!("Migration failed from version: {version}"),
        );

        log::debug!(target: "backend::state::AppHandle::migrate", "migrated_version: {:?}", migrated_version);

        log::trace!(target: "backend::state::AppHandle::migrate", "migrate end")
    }

    fn db<F, TResult>(&self, operation: F) -> TResult
    where
        F: FnOnce(&Database) -> TResult,
    {
        log::trace!(target: "backend::state::AppHandle::db", "db");

        let app_state: State<AppState> = self.state();
        let db_connection_guard = app_state
            .db
            .lock()
            .log_error_u("backend::state::AppHandle::db", "Lock database");

        let db = db_connection_guard
            .as_ref()
            .log_error_u("backend::state::AppHandle::db", "Get database connection");

        log::info!(target: "backend::state::AppHandle::db", "run operation");

        operation(db)
    }

    fn db_mut<F, TResult>(&self, operation: F) -> TResult
    where
        F: FnOnce(&mut Database) -> TResult,
    {
        log::trace!(target: "backend::state::AppHandle::db_mut", "db_mut");

        let app_state: State<AppState> = self.state();
        let mut db_connection_guard = app_state
            .db
            .lock()
            .log_error_u("backend::state::AppHandle::db_mut", "Lock database");

        let db = db_connection_guard
            .as_mut()
            .log_error_u("backend::state::AppHandle::db", "Get database connection");

        log::info!(target: "backend::state::AppHandle::db", "run operation");

        operation(db)
    }
}
