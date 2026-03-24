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
            crate::modules::journal::model::JournalStatus::Draft => "DRAFT",
            crate::modules::journal::model::JournalStatus::Posted => "POSTED",
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

    pub async fn get_journal_with_lines(
        &self,
        tx: &mut Transaction<'_, Sqlite>,
        journal_id: &str,
    ) -> Result<Journal, sqlx::Error> {

        let header = sqlx::query(
            "SELECT id, date, description, status FROM journal WHERE id = ?"
        )
            .bind(journal_id)
            .fetch_one(&mut **tx)
            .await?;

        let rows = sqlx::query(
            "SELECT id, journal_id, account_id, debit, credit
            FROM journal_line WHERE journal_id = ?"
        )
            .bind(journal_id)
            .fetch_all(&mut **tx)
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
            status: match header.get::<String, _>("status").as_str() {
                "POSTED" => JournalStatus::Posted,
                _ => JournalStatus::Draft,
            },
            lines,
        })
    }

    pub async fn insert_ledger_entries(
        &self,
        tx: &mut sqlx::Transaction<'_, sqlx::Sqlite>,
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
        tx: &mut sqlx::Transaction<'_, sqlx::Sqlite>,
        journal_id: &str,
        status: &str,
    ) -> Result<(), sqlx::Error> {
        sqlx::query(
            "UPDATE journal SET status = ? WHERE id = ?"
        )
        .bind(status)
        .bind(journal_id)
        .execute(&mut **tx)
        .await?;

        Ok(())
    }
}