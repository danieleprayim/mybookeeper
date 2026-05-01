#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use sqlx::SqlitePool;
use tauri::Manager;

mod modules;
mod shared;
mod states;
mod commands;
mod models;
mod infrastructure;

use commands::*;
use infrastructure::db;

use commands::journal::{create_journal, post_journal};

use crate::states::app_state::AppState;

use crate::modules::journal::{
    repository::JournalRepo,
    service::JournalService,
};

fn main() {
    env_logger::init();
    tauri::Builder::default()
        .setup(|app| {
            let app_handle = app.handle();

            // App directory setup
            let app_dir = app_handle
                .path()
                .app_data_dir()
                .expect("failed to get app dir");

            std::fs::create_dir_all(&app_dir)
                .expect("Failed to create app dir");

            // let db_path = app_dir.join("accounting.db");
            let db_path = std::path::PathBuf::from("accounting.db");
            log::info!("📦 Using SQLite DB at: {}", db_path.display());

            if !db_path.exists() {
                std::fs::File::create(&db_path)
                    .expect("Failed to create DB file");
            }

            // Initialize DB pool
            let pool: SqlitePool = tauri::async_runtime::block_on(async {
                db::init_pool(db_path)
                    .await
                    .expect("DB init failed")
            });
            app.manage(pool.clone());

            // CREATE repository
            let journal_repo = JournalRepo::new(pool.clone());

            // CREATE service
            let journal_service = JournalService::new(journal_repo);

            // REGISTER AppState
            app.manage(AppState {
                token: None,
                journal_service,
            });

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            create_coa,
            update_coa,
            list_all_coa,
            set_coa_active,
            create_journal,
            post_journal,
            list_all,
            get_by_id,
            register,
            login,
            logout,
            get_session,
            admin_only
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri");
    log::info!("🚀 Starting application... with db");
}