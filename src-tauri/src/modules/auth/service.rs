use crate::modules::auth::repository;
use crate::shared::crypto;
use sqlx::SqlitePool;

pub async fn register(
    pool: &SqlitePool,
    email: String,
    password: String,
) -> Result<String, String> {
    let hash = crypto::hash_password(&password);

    repository::create_user(pool, &email, &hash, "user")
        .await
        .map_err(|e| e.to_string())?;

    Ok("User created".into())
}

pub async fn login(
    pool: &SqlitePool,
    email: String,
    password: String,
) -> Result<String, String> {
    let user = repository::find_user(pool, &email)
        .await
        .map_err(|e| e.to_string())?
        .ok_or("User not found")?;

    let is_valid = crypto::verify_password(&password, &user.password);

    if !is_valid {
        return Err("Invalid credential".into());
    }

    let token = repository::create_session(pool, user.id)
        .await
        .map_err(|e| e.to_string())?;

    Ok(token)
}

pub async fn validate_session(
    pool: &SqlitePool,
    token: &str,
) -> Result<i64, String> {
    let session = repository::find_session(pool, token)
        .await
        .map_err(|e| e.to_string())?
        .ok_or("Invalid session")?;

    Ok(session.user_id)
}

pub async fn logout(
    pool: &SqlitePool,
    token: &str,
) -> Result<(), String> {
    repository::delete_session(pool, token)
        .await
        .map_err(|e| e.to_string())?;

    Ok(())
}