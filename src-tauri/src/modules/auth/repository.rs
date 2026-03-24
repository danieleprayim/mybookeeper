use crate::modules::auth::model::Session;
use crate::modules::auth::model::User;
use sqlx::SqlitePool;
use uuid::Uuid;

pub async fn create_user(
    pool: &SqlitePool,
    email: &str,
    password: &str,
    role: &str,
) -> Result<(), sqlx::Error> {
    sqlx::query(
        "INSERT INTO users (email, password, role) VALUES (?1, ?2, ?3)"
    )
    .bind(email)
    .bind(password)
    .bind(role)
    .execute(pool)
    .await?;

    Ok(())
}

pub async fn find_user(
    pool: &SqlitePool,
    email: &str,
) -> Result<Option<User>, sqlx::Error> {
    let user = sqlx::query_as::<_, User>(
        "SELECT id, email, password, role FROM users WHERE email = ?1"
    )
    .bind(email)
    .fetch_optional(pool)
    .await?;

    Ok(user)
}

pub async fn find_user_by_id(
    pool: &SqlitePool,
    user_id: i64,
) -> Result<Option<User>, sqlx::Error> {
    let user = sqlx::query_as::<_, User>(
        "SELECT id, email, password, role FROM users WHERE id = ?1"
    )
    .bind(user_id)
    .fetch_optional(pool)
    .await?;

    Ok(user)
}

pub async fn create_session(
    pool: &SqlitePool,
    user_id: i64,
) -> Result<String, sqlx::Error> {
    let token = Uuid::new_v4().to_string();

    sqlx::query(
        "INSERT INTO sessions (user_id, token) VALUES (?1, ?2)"
    )
    .bind(user_id)
    .bind(&token)
    .execute(pool)
    .await?;

    Ok(token)
}

pub async fn find_session(
    pool: &SqlitePool,
    token: &str,
) -> Result<Option<Session>, sqlx::Error> {
    let session = sqlx::query_as::<_, Session>(
        "SELECT user_id, token FROM sessions WHERE token = ?1"
    )
    .bind(token)
    .fetch_optional(pool)
    .await?;

    Ok(session)
}

pub async fn delete_session(
    pool: &SqlitePool,
    token: &str,
) -> Result<(), sqlx::Error> {
    sqlx::query(
        "DELETE FROM sessions WHERE token = ?1"
    )
    .bind(token)
    .execute(pool)
    .await?;

    Ok(())
}