use sqlx::SqlitePool;
use std::path::PathBuf;

pub async fn init_pool(db_path: PathBuf) -> Result<SqlitePool, sqlx::Error> {
    let db_url = format!("sqlite://{}", db_path.display());

    let pool = SqlitePool::connect(&db_url).await?;

    run_migrations(&pool).await?;
    seed_admin(&pool).await?;

    Ok(pool)
}

pub async fn run_migrations(pool: &SqlitePool) -> Result<(), sqlx::Error> {

    // USERS
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS users (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            email TEXT UNIQUE NOT NULL,
            password TEXT NOT NULL,
            role TEXT NOT NULL
        );
        "#
    )
    .execute(pool)
    .await?;

    // SESSIONS
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS sessions (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            user_id INTEGER NOT NULL,
            token TEXT NOT NULL
        );
        "#
    )
    .execute(pool)
    .await?;

    // COA
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS coa (
            account_id TEXT PRIMARY KEY,
            account_code TEXT NOT NULL,
            account_name TEXT NOT NULL,
            account_type TEXT NOT NULL,
            account_group_code TEXT NOT NULL,
            account_group_name TEXT,
            normal_account TEXT NOT NULL,
            description TEXT,
            parent_id TEXT,
            is_active INTEGER DEFAULT 1
        );
        "#
    )
    .execute(pool)
    .await?;

    // JOURNAL
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS journal (
            id TEXT PRIMARY KEY,
            date TEXT NOT NULL,
            description TEXT
        );
        "#
    )
    .execute(pool)
    .await?;

    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS journal_line (
            id TEXT PRIMARY KEY,
            journal_id TEXT,
            account_id TEXT,
            debit REAL DEFAULT 0,
            credit REAL DEFAULT 0
        );
        "#
    )
    .execute(pool)
    .await?;

    // PERIOD
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS period (
            id TEXT PRIMARY KEY,
            start_date TEXT,
            end_date TEXT,
            is_closed INTEGER DEFAULT 0
        );
        "#
    )
    .execute(pool)
    .await?;

    Ok(())
}

use crate::shared::crypto;

pub async fn seed_admin(pool: &SqlitePool) -> Result<(), sqlx::Error> {
    let admin_email = "admin@mail.com";
    let admin_password = "admin123";

    let existing = sqlx::query("SELECT id FROM users WHERE email = ?")
        .bind(admin_email)
        .fetch_optional(pool)
        .await?;

    if existing.is_some() {
        println!("Admin already exists");
        return Ok(());
    }

    let hashed = crypto::hash_password(admin_password);

    sqlx::query("INSERT INTO users (email, password, role) VALUES (?, ?, ?)")
        .bind(admin_email)
        .bind(hashed)
        .bind("admin")
        .execute(pool)
        .await?;

    println!("Admin created: {} / {}", admin_email, admin_password);

    Ok(())
}