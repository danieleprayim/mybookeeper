    use serde::{Serialize, Deserialize};
    use sqlx::{FromRow, Type};

    #[derive(Debug, Serialize, Deserialize, FromRow)]
    pub struct Journal {
        pub id: String,
        pub date: String,
        pub description: String,
        pub status: JournalStatus,
        pub lines: Vec<JournalLine>,
    }

    #[derive(Debug, Serialize, Deserialize, FromRow)]
    pub struct JournalLine {
        pub id: String,
        pub journal_id: String,
        pub account_id: String,
        pub debit: f64,
        pub credit: f64,
    }

    #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Type)]
    pub enum JournalStatus {
        Draft,
        Posted,
    }

    impl Journal {
        
        pub fn total_debit(&self) -> f64 {
            self.lines.iter().map(|l| l.debit).sum()
        }

        pub fn total_credit(&self) -> f64 {
            self.lines.iter().map(|l| l.credit).sum()
        }

        pub fn is_balanced(&self) -> bool {
            (self.total_debit() - self.total_credit()).abs() < 0.001
        }

        pub fn validate(&self) -> Result<(), String> {
            if self.lines.is_empty() {
                return Err("Journal must have at least one line".into());
            }

            if !self.is_balanced() {
                return Err("Journal not balanced".into());
            }

            Ok(())
        }

        #[allow(dead_code)]
        pub fn ensure_editable(&self) -> Result<(), String> {
            if self.status == JournalStatus::Posted {
                return Err("Cannot modify posted journal".into());
            }
            Ok(())
        }

        pub fn ensure_not_posted(&self) -> Result<(), String> {
            if self.status == JournalStatus::Posted {
                return Err("Journal already posted".into());
            }
            Ok(())
        }
    }
