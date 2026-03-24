use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct CreateCoaDTO {
    // pub account_id: String,
    pub account_code: String,
    pub account_name: String,
    pub account_type: String, 
    pub account_group_code: String,
    pub account_group_name: String,
    pub normal_account: String,
    pub description: Option<String>,
    pub parent_id: Option<String>,
    pub is_active: Option<bool>,
}

#[allow(dead_code)]
pub struct CreateAccountGroupDTO {
    // pub account_group_id: String,
    pub account_group_code: String,
    pub account_group_name: String,
    pub description: String,
    pub parent_id: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateCoaDTO {
    pub account_id: String,
    pub account_code: Option<String>,
    pub account_name: Option<String>,
    pub account_type: Option<String>,
    pub account_group_code: Option<String>,
    pub account_group_name: Option<String>,
    pub normal_account: Option<String>,
    pub description: Option<String>,
    pub parent_id: Option<String>,
    pub is_active: Option<bool>,
}