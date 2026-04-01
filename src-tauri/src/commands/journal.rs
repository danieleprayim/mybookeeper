use tauri::State;
use sqlx::SqlitePool;

use crate::modules::journal::{
    dto::CreateJournalDTO,
    repository::JournalRepo,
    service::JournalService,
    model::Journal,
};

fn get_service(pool: &SqlitePool) -> JournalService {
    let repo = JournalRepo::new(pool.clone());
    JournalService::new(repo)
}

#[tauri::command]
pub async fn create_journal(
    pool: State<'_, SqlitePool>,
    dto: CreateJournalDTO,
) -> Result<(), String> {
    get_service(pool.inner())
        .create_journal(dto)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn post_journal(
    pool: State<'_, SqlitePool>,
    journal_id: String,
) -> Result<(), String> {
    get_service(pool.inner())
        .post_journal(journal_id)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn list_all(
    pool: State<'_, SqlitePool>,
) -> Result<Vec<Journal>, String> {
    get_service(pool.inner())
        .list_all()
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_by_id(
    pool: State<'_, SqlitePool>,
    journal_id: String,
) -> Result<Journal, String> {
    get_service(pool.inner())
        .get_by_id(journal_id)
        .await
        .map_err(|e| e.to_string())
}