use sqlx::SqlitePool;
use std::path::PathBuf;

pub async fn init_pool(db_path: PathBuf) -> Result<SqlitePool, sqlx::Error> {
    
    let db_url = format!("sqlite://{}", db_path.display());

    let pool = SqlitePool::connect(&db_url).await?;

    sqlx::query("PRAGMA foreign_keys = ON;")
        .execute(&pool)
        .await?;

    run_migrations(&pool).await?;
    seed_account_group(&pool).await?;
    seed_admin(&pool).await?;
    seed_coa(&pool).await?;

    Ok(pool)
}

pub async fn run_migrations(pool: &SqlitePool) -> Result<(), sqlx::Error> {
    // Enable FK (VERY IMPORTANT for SQLite)
    sqlx::query("PRAGMA foreign_keys = ON;")
        .execute(pool)
        .await?;

    let mut tx = pool.begin().await?;

    // ================= ACCOUNT GROUP =================
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS account_group (
            account_group_id TEXT PRIMARY KEY,
            account_group_code TEXT NOT NULL,
            account_group_name TEXT NOT NULL,
            description TEXT,
            is_active INTEGER DEFAULT 1,
            parent_id TEXT,
            FOREIGN KEY(parent_id) REFERENCES account_group(account_group_id)
        );
        "#
    )
    .execute(&mut *tx)
    .await?;

    // ================= COA =================
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS coa (
            account_id TEXT PRIMARY KEY,
            account_code TEXT NOT NULL UNIQUE,
            account_name TEXT NOT NULL,
            account_type TEXT NOT NULL CHECK(account_type IN ('REAL','NOMINAL')),
            account_group_code TEXT NOT NULL,
            account_group_name TEXT NOT NULL,
            normal_account TEXT NOT NULL CHECK(normal_account IN ('DEBET','CREDIT')),
            description TEXT,
            parent_id TEXT,
            is_postable INTEGER DEFAULT 1,
            is_active INTEGER DEFAULT 1,
            FOREIGN KEY(parent_id) REFERENCES coa(account_id)
        );
        "#
    )
    .execute(&mut *tx)
    .await?;

    // indexes
    sqlx::query("CREATE INDEX IF NOT EXISTS idx_coa_parent ON coa(parent_id);")
        .execute(&mut *tx)
        .await?;

    sqlx::query("CREATE INDEX IF NOT EXISTS idx_coa_code ON coa(account_code);")
        .execute(&mut *tx)
        .await?;

    // ================= USERS =================
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS users (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            email TEXT UNIQUE NOT NULL,
            password TEXT NOT NULL,
            role TEXT NOT NULL
        );
        "#
    )
    .execute(&mut *tx)
    .await?;

    // ================= SESSIONS =================
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS sessions (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            user_id INTEGER NOT NULL,
            token TEXT NOT NULL
        );
        "#
    )
    .execute(&mut *tx)
    .await?;

    // ================= JOURNAL =================
    sqlx::query(
        r#" 
        CREATE TABLE IF NOT EXISTS journal (
            id TEXT PRIMARY KEY,
            date TEXT NOT NULL,
            description TEXT,
            status TEXT NOT NULL CHECK(status IN ('Draft','Posted')),
            is_active INTEGER DEFAULT 1
        );
        "#
    )
    .execute(&mut *tx)
    .await?;

    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS journal_line (
            id TEXT PRIMARY KEY,
            journal_id TEXT NOT NULL,
            account_id TEXT NOT NULL,
            debit REAL DEFAULT 0,
            credit REAL DEFAULT 0,
            CHECK (debit > 0 OR credit > 0),
            FOREIGN KEY(journal_id) REFERENCES journal(id),
            FOREIGN KEY(account_id) REFERENCES coa(account_id)
        );
        "#
    )
    .execute(&mut *tx)
    .await?;

    sqlx::query("CREATE INDEX IF NOT EXISTS idx_journal_line_account ON journal_line(account_id);")
        .execute(&mut *tx)
        .await?;

    // ================= PERIOD =================
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS period (
            id TEXT PRIMARY KEY,
            start_date TEXT,
            end_date TEXT,
            is_closed INTEGER DEFAULT 0
        );
        "#
    )
    .execute(&mut *tx)
    .await?;

    tx.commit().await?;

    Ok(())
}

use crate::shared::crypto;

pub async fn seed_admin(pool: &SqlitePool) -> Result<(), sqlx::Error> {
    let admin_email = "admin@mail.com";
    let admin_password = "admin123";

    let existing = sqlx::query("SELECT id FROM users WHERE email = ?")
        .bind(admin_email)
        .fetch_optional(pool)
        .await?;

    if existing.is_some() {
        println!("Admin already exists");
        return Ok(());
    }

    let hashed = crypto::hash_password(admin_password);

    sqlx::query("INSERT INTO users (email, password, role) VALUES (?, ?, ?)")
        .bind(admin_email)
        .bind(hashed)
        .bind("admin")
        .execute(pool)
        .await?;

    println!("Admin created: {} / {}", admin_email, admin_password);

    Ok(())
}

pub async fn seed_coa(pool: &SqlitePool) -> Result<(), sqlx::Error> {
    // Check if COA already exists
    let existing: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM coa")
        .fetch_one(pool)
        .await?;

    if existing.0 > 0 {
        println!("COA already seeded");
        return Ok(());
    }

    println!("Seeding COA...");

    let mut tx = pool.begin().await?;

    // ================= ASSETS =================
    sqlx::query(
        r#"
        INSERT INTO coa (
            account_id, 
            account_code, 
            account_name, 
            account_type,
            account_group_code, 
            account_group_name, 
            normal_account,
            description, 
            parent_id, 
            is_active
        ) VALUES
        ('1','1000','Assets','REAL','ASSET','ASSET','DEBET','Root Assets',NULL,1),
        ('2','1100','Current Assets','REAL','ASSET','ASSET','DEBET',NULL,'1',1),
        ('3','1101','Cash','REAL','ASSET','ASSET','DEBET',NULL,'2',1),
        ('4','1102','Bank Account','REAL','ASSET','ASSET','DEBET',NULL,'2',1),
        ('5','1103','Petty Cash','REAL','ASSET','ASSET','DEBET',NULL,'2',1),
        ('6','1200','Accounts Receivable','REAL','ASSET','ASSET','DEBET',NULL,'2',1),
        ('7','1201','Trade Receivables','REAL','ASSET','ASSET','DEBET',NULL,'6',1),
        ('8','1202','Allowance for Doubtful Accounts','REAL','ASSET','ASSET','CREDIT',NULL,'6',1),
        ('9','1300','Inventory','REAL','ASSET','ASSET','DEBET',NULL,'2',1),
        ('10','1301','Raw Materials','REAL','ASSET','ASSET','DEBET',NULL,'9',1),
        ('11','1302','Finished Goods','REAL','ASSET','ASSET','DEBET',NULL,'9',1),
        ('12','1400','Prepaid Expenses','REAL','ASSET','ASSET','DEBET',NULL,'2',1),
        ('13','1500','Fixed Assets','REAL','ASSET','ASSET','DEBET',NULL,'1',1),
        ('14','1501','Equipment','REAL','ASSET','ASSET','DEBET',NULL,'13',1),
        ('15','1502','Vehicles','REAL','ASSET','ASSET','DEBET',NULL,'13',1),
        ('16','1503','Buildings','REAL','ASSET','ASSET','DEBET',NULL,'13',1),
        ('17','1600','Accumulated Depreciation','REAL','ASSET','ASSET','CREDIT',NULL,'13',1),
        ('18','1601','Accum Depreciation - Equipment','REAL','ASSET','ASSET','CREDIT',NULL,'17',1),
        ('19','1602','Accum Depreciation - Vehicles','REAL','ASSET','ASSET','CREDIT',NULL,'17',1)
        "#
    )
    .execute(&mut *tx)
    .await?;

    // ================= LIABILITIES =================
    sqlx::query(
        r#"
        INSERT INTO coa (
            account_id, 
            account_code, 
            account_name, 
            account_type,
            account_group_code, 
            account_group_name, 
            normal_account,
            description, 
            parent_id, 
            is_active
        ) VALUES
        ('20','2000','Liabilities','REAL','LIABILITY','LIABILITY','CREDIT',NULL,NULL,1),
        ('21','2100','Current Liabilities','REAL','LIABILITY','LIABILITY','CREDIT',NULL,'20',1),
        ('22','2101','Accounts Payable','REAL','LIABILITY','LIABILITY','CREDIT',NULL,'21',1),
        ('23','2102','Accrued Expenses','REAL','LIABILITY','LIABILITY','CREDIT',NULL,'21',1),
        ('24','2103','Tax Payable','REAL','LIABILITY','LIABILITY','CREDIT',NULL,'21',1),
        ('25','2200','Long-term Liabilities','REAL','LIABILITY','LIABILITY','CREDIT',NULL,'20',1),
        ('26','2201','Bank Loan','REAL','LIABILITY','LIABILITY','CREDIT',NULL,'25',1)
        "#
    )
    .execute(&mut *tx)
    .await?;

    // ================= EQUITY =================
    sqlx::query(
        r#"
        INSERT INTO coa (
            account_id, 
            account_code, 
            account_name, 
            account_type,
            account_group_code, 
            account_group_name, 
            normal_account,
            description, 
            parent_id, 
            is_active
        ) VALUES
        ('30','3000','Equity','REAL','EQUITY','EQUITY','CREDIT',NULL,NULL,1),
        ('31','3100','Owner Capital','REAL','EQUITY','EQUITY','CREDIT',NULL,'30',1),
        ('32','3200','Retained Earnings','REAL','EQUITY','EQUITY','CREDIT',NULL,'30',1),
        ('33','3300','Drawing','REAL','EQUITY','EQUITY','DEBET',NULL,'30',1);
        "#
    )
    .execute(&mut *tx)
    .await?;

    // ================= INCOME =================
    sqlx::query(
        r#"
        INSERT INTO coa (
            account_id, 
            account_code, 
            account_name, 
            account_type,
            account_group_code, 
            account_group_name, 
            normal_account,
            description, 
            parent_id, 
            is_active
        ) VALUES
        ('40','4000','Income','NOMINAL','INCOME','INCOME','CREDIT',NULL,NULL,1),
        ('41','4100','Sales Revenue','NOMINAL','INCOME','INCOME','CREDIT',NULL,'40',1),
        ('42','4200','Service Revenue','NOMINAL','INCOME','INCOME','CREDIT',NULL,'40',1),
        ('43','4300','Other Income','NOMINAL','INCOME','INCOME','CREDIT',NULL,'40',1)
        "#
    )
    .execute(&mut *tx)
    .await?;

    // ================= EXPENSE =================
    sqlx::query(
        r#"
        INSERT INTO coa (
            account_id, 
            account_code, 
            account_name, 
            account_type,
            account_group_code, 
            account_group_name, 
            normal_account,
            description, 
            parent_id, 
            is_active
        ) VALUES
        ('50','5000','Expenses','NOMINAL','EXPENSE','EXPENSE','DEBET',NULL,NULL,1),
        ('51','5100','Operating Expenses','NOMINAL','EXPENSE','EXPENSE','DEBET',NULL,'50',1),
        ('52','5101','Salary Expense','NOMINAL','EXPENSE','EXPENSE','DEBET',NULL,'51',1),
        ('53','5102','Rent Expense','NOMINAL','EXPENSE','EXPENSE','DEBET',NULL,'51',1),
        ('54','5103','Utilities Expense','NOMINAL','EXPENSE','EXPENSE','DEBET',NULL,'51',1),
        ('55','5104','Office Supplies','NOMINAL','EXPENSE','EXPENSE','DEBET',NULL,'51',1),
        ('56','5200','Cost of Goods Sold','NOMINAL','EXPENSE','EXPENSE','DEBET',NULL,'50',1),
        ('57','5201','COGS - Materials','NOMINAL','EXPENSE','EXPENSE','DEBET',NULL,'56',1),
        ('58','5300','Depreciation Expense','NOMINAL','EXPENSE','EXPENSE','DEBET',NULL,'50',1)
        "#
    )
    .execute(&mut *tx)
    .await?;

    tx.commit().await?;

    println!("COA seeded successfully");

    Ok(())
}

pub async fn seed_account_group(pool: &SqlitePool) -> Result<(), sqlx::Error> {
    let count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM account_group")
        .fetch_one(pool)
        .await?;

    if count.0 > 0 {
        return Ok(());
    }

    sqlx::query(
        r#"
        INSERT INTO account_group (
            account_group_id,
            account_group_code,
            account_group_name,
            description,
            is_active,
            parent_id
        ) VALUES
        ('AG1','ASSET','Assets','Assets',1,NULL),
        ('AG2','LIABILITY','Liabilities','Liabilities',1,NULL),
        ('AG3','EQUITY','Equity','Equity',1,NULL),
        ('AG4','INCOME','Income','Income',1,NULL),
        ('AG5','EXPENSE','Expense','Expense',1,NULL)
        "#
    )
    .execute(pool)
    .await?;

    Ok(())
}