use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateJournalDTO {
    pub journal_id: String,
    pub date: String,
    pub description: String,
    pub lines: Vec<CreateJournalLineDTO>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateJournalLineDTO {
    pub account_id: String,
    pub debit: f64,
    pub credit: f64,
}