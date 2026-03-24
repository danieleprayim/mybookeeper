use serde::{Serialize, Deserialize};
use sqlx::{FromRow, Type};

// ================= ENUM: AccountType =================
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Type)]
#[sqlx(type_name = "TEXT", rename_all = "UPPERCASE")]
pub enum AccountType {
    Nominal,
    Real,
}

// ================= ENUM: NormalAccount =================
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Type)]
#[sqlx(type_name = "TEXT", rename_all = "UPPERCASE")]
pub enum NormalAccount {
    Debet,
    Credit,
}

// ================= ENUM: account_group_code =================
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Type)]
#[sqlx(type_name = "TEXT", rename_all = "UPPERCASE")]
pub enum AccountGroupCode {
    Asset,
    Liability,
    Equity,
    Income,
    Expense,
}

// ================= ENUM: account_group_name =================
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Type)]
#[sqlx(type_name = "TEXT", rename_all = "UPPERCASE")]
pub enum AccountGroupName {
    Asset,
    Liability,
    Equity,
    Income,
    Expense,
}

// ================= ENTITY: COA =================
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Coa {
    pub account_id: String,
    pub account_code: String,
    pub account_name: String,
    pub account_type: AccountType,
    pub account_group_code: AccountGroupCode,
    pub account_group_name: AccountGroupName,
    pub normal_account: NormalAccount,
    pub description: Option<String>,
    pub is_active: bool,
    pub parent_id: Option<String>,
}

// ================= ENTITY: ACCOUNT GROUP =================
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct AccountGroup {
    pub account_group_id: String,
    pub account_group_code: AccountGroupCode,
    pub account_group_name: AccountGroupName,
    pub description: String,
    pub is_active: bool,
    pub parent_id: Option<String>,
}