use crate::modules::auth::{dto, guard, service};
use sqlx::SqlitePool;
use tauri::{State};


#[tauri::command]   
pub async fn register(
    pool: State<'_, SqlitePool>,
    dto: dto::RegisterDto,
) -> Result<String, String> {
    service::register(&pool, dto.email, dto.password).await
}

#[tauri::command]
pub async fn login(
    pool: State<'_, SqlitePool>,
    dto: dto::LoginDto,
) -> Result<String, String> {
    service::login(&pool, dto.email, dto.password).await
}

#[tauri::command]
pub async fn logout(
    pool: State<'_, SqlitePool>,
    token: String,
) -> Result<String, String> {
    service::logout(&pool, &token).await?;
    Ok("Logged out".into())
}

#[tauri::command]
pub async fn get_session(
    pool: State<'_, SqlitePool>,
    token: String,
) -> Result<String, String> {
    service::validate_session(&pool, &token)
        .await
        .map_err(|e| e.to_string())?;

    Ok("Valid session".into())
}

#[tauri::command]
pub async fn admin_only(
    pool: State<'_, SqlitePool>,
    token: String,
) -> Result<String, String> {
    guard::require_role(&pool, &token, "admin")
        .await
        .map_err(|e| e.to_string())?;

    Ok("Welcome admin".into())
}