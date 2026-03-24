use tauri::State;
use sqlx::SqlitePool;
use crate::states::app_state::AppState;

use crate::modules::journal::{
    dto::CreateJournalDTO,
    repository::JournalRepo,
    service::JournalService,
};

#[tauri::command]
pub async fn create_journal(
    pool: State<'_, SqlitePool>,
    dto: CreateJournalDTO,
) -> Result<(), String> {
    let repo = JournalRepo::new(pool.inner().clone());
    let service = JournalService::new(repo);

    service.create_journal(dto).await
}

#[tauri::command]
pub async fn post_journal(
    state: State<'_, AppState>,
    journal_id: String,
) -> Result<(), String> {
    state.journal_service.post_journal(journal_id).await
}