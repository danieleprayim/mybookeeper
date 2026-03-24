use crate::modules::journal::service::JournalService;

pub struct AppState {
    #[allow(dead_code)]
    pub token: Option<String>,
    pub journal_service: JournalService,
}