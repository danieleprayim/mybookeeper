use crate::modules::coa::model::{Coa, AccountGroup};
use sqlx::SqlitePool;

pub struct CoaRepo {
    pool: SqlitePool,
}

impl CoaRepo {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }

    // ================= INSERT COA =================
    pub async fn insert_coa(&self, coa: &Coa) -> Result<(), sqlx::Error> {
        sqlx::query(
            r#"
            INSERT INTO coa (
                account_id, account_code, account_name, account_type,
                account_group_code, account_group_name, normal_account,
                description, is_active, parent_id
            ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
            "#,
        )
        .bind(&coa.account_id)
        .bind(&coa.account_code)
        .bind(&coa.account_name)
        .bind(&coa.account_type)
        .bind(&coa.account_group_code)
        .bind(&coa.account_group_name)
        .bind(&coa.normal_account)
        .bind(&coa.description)
        .bind(coa.is_active)
        .bind(&coa.parent_id)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    // ================= FIND BY ID =================
    pub async fn find_by_id(&self, account_id: &str) -> Result<Option<Coa>, sqlx::Error> {
        let result = sqlx::query_as::<_, Coa>(
            r#"
            SELECT * FROM coa WHERE account_id = ?
            "#
        )
        .bind(account_id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(result)
    }

    // list all
    pub async fn list_all_coa(&self) -> Result<Vec<Coa>, sqlx::Error> {
        let result = sqlx::query_as::<_, Coa>(
            r#"
            SELECT * FROM coa
            ORDER BY account_code
            "#
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(result)
    }
    
    // ================= UPDATE COA =================
    pub async fn update_coa(&self, coa: &Coa) -> Result<(), sqlx::Error> {
        sqlx::query(
            r#"
            UPDATE coa
            SET
                account_name = ?,
                account_type = ?,
                account_group_code = ?,
                account_group_name = ?,
                normal_account = ?,
                description = ?,
                parent_id = ?
            WHERE account_id = ?
            "#
        )
        .bind(&coa.account_name)
        .bind(&coa.account_type)
        .bind(&coa.account_group_code)
        .bind(&coa.account_group_name)
        .bind(&coa.normal_account)
        .bind(&coa.description)
        .bind(&coa.parent_id)
        .bind(&coa.account_id)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    // ================= SET ACTIVE =================
    pub async fn set_active_status(
        &self,
        account_id: &str,
        is_active: bool,
    ) -> Result<(), sqlx::Error> {
        sqlx::query(
            "UPDATE coa SET is_active = ? WHERE account_id = ?"
        )
        .bind(is_active)
        .bind(account_id)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    // ================= ACCOUNT GROUP =================
    #[allow(dead_code)]
    pub async fn insert_account_group(&self, group: &AccountGroup) -> Result<(), sqlx::Error> {
        sqlx::query(
            r#"
            INSERT INTO account_group (
                account_group_id, account_group_code, account_group_name,
                description, is_active, parent_id
            ) VALUES (?, ?, ?, ?, ?, ?)
            "#,
        )
        .bind(&group.account_group_id)
        .bind(&group.account_group_code)
        .bind(&group.account_group_name)
        .bind(&group.description)
        .bind(group.is_active)
        .bind(&group.parent_id)
        .execute(&self.pool)
        .await?;

        Ok(())
    }
}