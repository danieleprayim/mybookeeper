use tauri::State;
// use serde::Serialize;
use sqlx::SqlitePool;
use sqlx::FromRow;

#[derive(Debug, serde::Serialize, FromRow)]
pub struct TrialBalanceRow {
    pub account_id: String,
    pub total_debit: f64,
    pub total_credit: f64,
}

#[tauri::command]
pub async fn get_trial_balance(
    pool: State<'_, SqlitePool>,
) -> Result<Vec<TrialBalanceRow>, String> {

    let rows = sqlx::query_as::<_, TrialBalanceRow>(
        "
        SELECT 
            account_id,
            SUM(debit) as total_debit,
            SUM(credit) as total_credit,
            SUM(debit - credit) as balance
        FROM journal_line
        GROUP BY account_id
        "
    )
    .fetch_all(pool.inner())
    .await
    .map_err(|e| e.to_string())?;

    Ok(rows)
}

#[derive(Debug, serde::Serialize, FromRow)]
pub struct LedgerRow {
    pub journal_id: String,
    pub date: String,
    pub description: Option<String>,
    pub debit: f64,
    pub credit: f64,
}

#[tauri::command]
pub async fn get_ledger(
    pool: State<'_, SqlitePool>,
    account_id: String,
) -> Result<Vec<LedgerRow>, String> {

    let rows = sqlx::query_as::<_, LedgerRow>(
        "
        SELECT 
            jl.journal_id,
            j.date,
            j.description,
            jl.debit,
            jl.credit
        FROM journal_line jl
        JOIN journal j ON jl.journal_id = j.id
        WHERE jl.account_id = ?
        ORDER BY j.date ASC
        "
    )
    .bind(account_id)
    .fetch_all(pool.inner())
    .await
    .map_err(|e| e.to_string())?;

    Ok(rows)
}