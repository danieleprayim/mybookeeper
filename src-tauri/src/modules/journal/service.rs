use uuid::Uuid;

use crate::modules::journal::{
    dto::CreateJournalDTO,
    model::{Journal, JournalLine, JournalStatus},
    repository::JournalRepo,
};

pub struct JournalService {
    repo: JournalRepo,
}

impl JournalService {
    pub fn new(repo: JournalRepo) -> Self {
        Self { repo }
    }

    pub async fn create_journal(&self, dto: CreateJournalDTO) -> Result<(), String> {
        //  DTO → MODEL
        let lines: Vec<JournalLine> = dto.lines.into_iter().map(|l| JournalLine {
            id: Uuid::new_v4().to_string(),
            journal_id: dto.journal_id.clone(),
            account_id: l.account_id,
            debit: l.debit,
            credit: l.credit,
        }).collect();

        let journal = Journal {
            id: dto.journal_id,
            date: dto.date,
            description: dto.description,
            status: JournalStatus::Draft,
            lines,
        };

        // DOMAIN VALIDATION
        journal.validate()?;

        // TRANSACTION
        let mut tx = self.repo.begin_tx().await.map_err(|e| e.to_string())?;

        self.repo
            .insert_journal(&mut tx, &journal)
            .await
            .map_err(|e| e.to_string())?;

        self.repo
            .insert_lines(&mut tx, &journal.lines)
            .await
            .map_err(|e| e.to_string())?;

        tx.commit().await.map_err(|e| e.to_string())?;

        Ok(())
    }

    pub async fn post_journal(&self, journal_id: String) -> Result<(), String> {
        let mut tx = self.repo.begin_tx().await.map_err(|e| e.to_string())?;

        // 1. Load journal
        let journal = self.repo
            .get_journal_with_lines(&mut tx, &journal_id)
            .await
            .map_err(|e| e.to_string())?;

        // 2. Domain validations
        journal.validate()?;
        journal.ensure_not_posted()?;

        // 3. Insert ledger
        self.repo
            .insert_ledger_entries(&mut tx, &journal)
            .await
            .map_err(|e| e.to_string())?;

        // 4. Update status
        self.repo
            .update_status(&mut tx, &journal.id, "POSTED")
            .await
            .map_err(|e| e.to_string())?;

        tx.commit().await.map_err(|e| e.to_string())?;

        Ok(())
    }
}