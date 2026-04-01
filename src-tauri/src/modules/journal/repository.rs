use std::collections::HashMap;
use sqlx::{SqlitePool, Transaction, Sqlite, Row};

use crate::modules::journal::model::{Journal, JournalLine, JournalStatus};

pub struct JournalRepo {
    pool: SqlitePool,
}

impl JournalRepo {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }

    pub async fn begin_tx(&self) -> Result<Transaction<'_, Sqlite>, sqlx::Error> {
        self.pool.begin().await
    }

    fn map_status(status: &str) -> JournalStatus {
        match status {
            "POSTED" => JournalStatus::Posted,
            _ => JournalStatus::Draft,
        }
    }

    // =========================
    // INSERT
    // =========================
    pub async fn insert_journal(
        &self,
        tx: &mut Transaction<'_, Sqlite>,
        journal: &Journal,
    ) -> Result<(), sqlx::Error> {
        sqlx::query(
            "INSERT INTO journal (id, date, description, status)
             VALUES (?, ?, ?, ?)"
        )
        .bind(&journal.id)
        .bind(&journal.date)
        .bind(&journal.description)
        .bind(match journal.status {
            JournalStatus::Draft => "DRAFT",
            JournalStatus::Posted => "POSTED",
        })
        .execute(&mut **tx)
        .await?;

        Ok(())
    }

    pub async fn insert_lines(
        &self,
        tx: &mut Transaction<'_, Sqlite>,
        lines: &[JournalLine],
    ) -> Result<(), sqlx::Error> {
        for line in lines {
            sqlx::query(
                "INSERT INTO journal_line (id, journal_id, account_id, debit, credit)
                 VALUES (?, ?, ?, ?, ?)"
            )
            .bind(&line.id)
            .bind(&line.journal_id)
            .bind(&line.account_id)
            .bind(line.debit)
            .bind(line.credit)
            .execute(&mut **tx)
            .await?;
        }

        Ok(())
    }

    // =========================
    // READ (NO TX)
    // =========================
    pub async fn get_all_journals_with_lines(
        &self,
    ) -> Result<Vec<Journal>, sqlx::Error> {
        let headers = sqlx::query(
            "SELECT id, date, description, status FROM journal ORDER BY date DESC"
        )
        .fetch_all(&self.pool)
        .await?;

        let rows = sqlx::query(
            "SELECT id, journal_id, account_id, debit, credit FROM journal_line"
        )
        .fetch_all(&self.pool)
        .await?;

        let mut lines_map: HashMap<String, Vec<JournalLine>> = HashMap::new();

        for row in rows {
            let line = JournalLine {
                id: row.get("id"),
                journal_id: row.get("journal_id"),
                account_id: row.get("account_id"),
                debit: row.get("debit"),
                credit: row.get("credit"),
            };

            lines_map
                .entry(line.journal_id.clone())
                .or_default()
                .push(line);
        }

        let mut journals = Vec::new();

        for h in headers {
            let id: String = h.get("id");

            journals.push(Journal {
                id: id.clone(),
                date: h.get("date"),
                description: h.get("description"),
                status: Self::map_status(&h.get::<String, _>("status")),
                lines: lines_map.remove(&id).unwrap_or_default(),
            });
        }

        Ok(journals)
    }

    pub async fn get_journal_with_lines_by_id(
        &self,
        journal_id: &str,
    ) -> Result<Journal, sqlx::Error> {
        let header = sqlx::query(
            "SELECT id, date, description, status FROM journal WHERE id = ?"
        )
        .bind(journal_id)
        .fetch_one(&self.pool)
        .await?;

        let rows = sqlx::query(
            "SELECT id, journal_id, account_id, debit, credit
             FROM journal_line WHERE journal_id = ?"
        )
        .bind(journal_id)
        .fetch_all(&self.pool)
        .await?;

        let lines = rows.into_iter().map(|row| JournalLine {
            id: row.get("id"),
            journal_id: row.get("journal_id"),
            account_id: row.get("account_id"),
            debit: row.get("debit"),
            credit: row.get("credit"),
        }).collect();

        Ok(Journal {
            id: header.get("id"),
            date: header.get("date"),
            description: header.get("description"),
            status: Self::map_status(&header.get::<String, _>("status")),
            lines,
        })
    }

    // =========================
    // POSTING
    // =========================
    pub async fn insert_ledger_entries(
        &self,
        tx: &mut Transaction<'_, Sqlite>,
        journal: &Journal,
    ) -> Result<(), sqlx::Error> {
        for line in &journal.lines {
            sqlx::query(
                "INSERT INTO ledger (id, journal_id, account_id, debit, credit, date)
                 VALUES (?, ?, ?, ?, ?, ?)"
            )
            .bind(uuid::Uuid::new_v4().to_string())
            .bind(&journal.id)
            .bind(&line.account_id)
            .bind(line.debit)
            .bind(line.credit)
            .bind(&journal.date)
            .execute(&mut **tx)
            .await?;
        }

        Ok(())
    }

    pub async fn update_status(
        &self,
        tx: &mut Transaction<'_, Sqlite>,
        journal_id: &str,
        status: &str,
    ) -> Result<(), sqlx::Error> {
        sqlx::query("UPDATE journal SET status = ? WHERE id = ?")
            .bind(status)
            .bind(journal_id)
            .execute(&mut **tx)
            .await?;

        Ok(())
    }
}