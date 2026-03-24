use tauri::State;
use sqlx::SqlitePool;

use crate::modules::coa::{
    repository::CoaRepo,
    service::CoaService,
    dto::{CreateCoaDTO, UpdateCoaDTO},
    model::Coa,
};

fn get_service(pool: &SqlitePool) -> CoaService {
    let repo = CoaRepo::new(pool.clone());
    CoaService::new(repo)
}

#[tauri::command]
pub async fn create_coa(
    pool: State<'_, SqlitePool>,
    dto: CreateCoaDTO,
) -> Result<(), String> {
    let service = get_service(pool.inner());

    service
        .create_coa(dto)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn update_coa(
    pool: State<'_, SqlitePool>,
    dto: UpdateCoaDTO,
) -> Result<(), String> {
    let service = get_service(pool.inner());

    service
        .update_coa(dto)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn set_coa_active(
    pool: State<'_, SqlitePool>,
    account_id: String,
    is_active: bool,
) -> Result<(), String> {
    let service = get_service(pool.inner());

    service
        .set_active_status(&account_id, is_active)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn list_all_coa(
    pool: State<'_, SqlitePool>,
) -> Result<Vec<Coa>, String> {
    let service = get_service(pool.inner());

    service
        .list_all_coa()
        .await
        .map_err(|e| e.to_string())
}