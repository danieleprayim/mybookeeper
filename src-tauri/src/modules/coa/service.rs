use crate::modules::coa::{
    dto::{CreateCoaDTO, UpdateCoaDTO},
    model::{Coa, AccountType, NormalAccount, AccountGroupCode, AccountGroupName},
    repository::CoaRepo,
};
use crate::shared::error::AppError;

pub struct CoaService {
    repo: CoaRepo,
}

impl CoaService {
    pub fn new(repo: CoaRepo) -> Self {
        Self { repo }
    }

    // create coa method
    pub async fn create_coa(&self, dto: CreateCoaDTO) -> Result<(), AppError> {
        self.validate_create(&dto)?;

        let account_type = Self::parse_account_type(&dto.account_type)?;
        let normal_account = Self::parse_normal_account(&dto.normal_account)?;
        let account_group_code = Self::parse_account_group_code(&dto.account_group_code)?;
        let account_group_name = Self::parse_account_group_name(&dto.account_group_name)?;
        let account_id = uuid::Uuid::new_v4().to_string();

        let coa = Coa {
            account_id,
            account_code: dto.account_code,
            account_name: dto.account_name,
            account_type,
            account_group_code,
            account_group_name,
            normal_account,
            description: dto.description,
            parent_id: dto.parent_id,
            is_active: dto.is_active.unwrap_or(true),
        };

        self.repo
            .insert_coa(&coa)
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(())
    }

    // update coa method
    pub async fn update_coa(&self, dto: UpdateCoaDTO) -> Result<(), AppError> {
        // 1. Fetch existing data
        let mut existing = self.repo
            .find_by_id(&dto.account_id)
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))?
            .ok_or(AppError::NotFound("COA not found".into()))?;

        // 2. Merge fields (ONLY update if provided)
        if let Some(code) = dto.account_code {
            if code.trim().is_empty() {
                return Err(AppError::ValidationError("Account code empty".into()));
            }
            existing.account_code = code;
        }

        if let Some(name) = dto.account_name {
            if name.trim().is_empty() {
                return Err(AppError::ValidationError("Account name empty".into()));
            }
            existing.account_name = name;
        }

        if let Some(t) = dto.account_type {
            existing.account_type = Self::parse_account_type(&t)?;
        }

        if let Some(n) = dto.normal_account {
            existing.normal_account = Self::parse_normal_account(&n)?;
        }

        if let Some(n) = dto.account_group_code {
            existing.account_group_code = Self::parse_account_group_code(&n)?;
        }

        if let Some(n) = dto.account_group_name {
            existing.account_group_name = Self::parse_account_group_name(&n)?;
        }

        if let Some(desc) = dto.description {
            existing.description = Some(desc);
        }

        if let Some(parent_id) = dto.parent_id {
            existing.parent_id = Some(parent_id);
        }

        if let Some(active) = dto.is_active {
            existing.is_active = active;
        }

        // 3. Save updated
        self.repo
            .update_coa(&existing)
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(())
    }

    pub async fn list_all_coa(&self) -> Result<Vec<Coa>, AppError> {
        let result = self.repo
            .list_all_coa()
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(result)
    }

    // set coa status method
    pub async fn set_active_status(
        &self,
        account_id: &str,
        is_active: bool,
    ) -> Result<(), AppError> {
        self.repo
            .set_active_status(account_id, is_active)
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))
    }

    // validation method
    fn validate_create(&self, dto: &CreateCoaDTO) -> Result<(), AppError> {
        if dto.account_code.trim().is_empty() {
            return Err(AppError::ValidationError("Account code empty".into()));
        }

        if dto.account_name.trim().is_empty() {
            return Err(AppError::ValidationError("Account name empty".into()));
        }

        Ok(())
    }

    // parser method
    fn parse_account_type(value: &str) -> Result<AccountType, AppError> {
        match value {
            "NOMINAL" => Ok(AccountType::Nominal),
            "REAL" => Ok(AccountType::Real),
            _ => Err(AppError::ValidationError("Invalid account type".into())),
        }
    }

    fn parse_normal_account(value: &str) -> Result<NormalAccount, AppError> {
        match value {
            "DEBET" => Ok(NormalAccount::Debet),
            "CREDIT" => Ok(NormalAccount::Credit),
            _ => Err(AppError::ValidationError("Invalid normal account".into())),
        }
    }

    fn parse_account_group_code(value: &str) -> Result<AccountGroupCode, AppError> {
        match value {
            "ASSET" => Ok(AccountGroupCode::Asset),
            "LIABILITY" => Ok(AccountGroupCode::Liability),
            "EQUITY" => Ok(AccountGroupCode::Equity),
            "INCOME" => Ok(AccountGroupCode::Income),
            "EXPENSE" => Ok(AccountGroupCode::Expense),
            _ => Err(AppError::ValidationError("Invalid account group code".into())),
        }
    }

    fn parse_account_group_name(value: &str) -> Result<AccountGroupName, AppError> {
        match value {
            "ASSET" => Ok(AccountGroupName::Asset),
            "LIABILITY" => Ok(AccountGroupName::Liability),
            "EQUITY" => Ok(AccountGroupName::Equity),
            "INCOME" => Ok(AccountGroupName::Income),
            "EXPENSE" => Ok(AccountGroupName::Expense),
            _ => Err(AppError::ValidationError("Invalid account group name".into())),
        }
    }
}