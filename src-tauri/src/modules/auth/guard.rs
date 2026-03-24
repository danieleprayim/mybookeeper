use crate::modules::auth::{repository, service};
use sqlx::SqlitePool;

pub async fn require_role(
    pool: &SqlitePool,
    token: &str,
    required_role: &str,
) -> Result<(), String> {
    // validate session → get user_id
    let user_id = service::validate_session(pool, token).await?;

    // get user by id
    let user = repository::find_user_by_id(pool, user_id)
        .await
        .map_err(|e| e.to_string())?
        .ok_or("User not found")?;

    // check role
    if user.role != required_role {
        return Err("Forbidden".into());
    }

    Ok(())
}