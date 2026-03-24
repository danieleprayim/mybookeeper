use tauri::State;
use sqlx::SqlitePool;

#[tauri::command]
pub async fn close_period(
    pool: State<'_, SqlitePool>,
    period_id: String,
) -> Result<(), String> {

    sqlx::query(
        "UPDATE period SET is_closed = 1 WHERE id = ?"
    )
    .bind(period_id)
    .execute(pool.inner())
    .await
    .map_err(|e| e.to_string())?;

    Ok(())
}