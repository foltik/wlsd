use std::path::Path;

use anyhow::Result;
use lettre::message::Mailbox;
use rand::{rngs::OsRng, Rng as _};
use sqlx::{migrate::MigrateDatabase, Sqlite, SqlitePool};

#[derive(Clone)]
pub struct Db {
    pool: SqlitePool,
}

#[derive(sqlx::FromRow, serde::Serialize)]
pub struct User {
    pub id: i64,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub created_at: String,
}

impl Db {
    pub async fn connect(file: &Path) -> Result<Self> {
        let url = format!("sqlite://{}", file.display());
        if !Sqlite::database_exists(&url).await? {
            Sqlite::create_database(&url).await?;
        }
        let pool = SqlitePool::connect(&url).await?;

        let db = Self { pool };
        db.migrate().await?;
        Ok(db)
    }

    async fn migrate(&self) -> Result<()> {
        sqlx::query(
            "CREATE TABLE IF NOT EXISTS users ( \
                id INTEGER PRIMARY KEY NOT NULL, \
                first_name TEXT NOT NULL, \
                last_name TEXT NOT NULL, \
                email TEXT NOT NULL, \
                created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP \
            )",
        )
        .execute(&self.pool)
        .await?;

        sqlx::query(
            "CREATE TABLE IF NOT EXISTS login_tokens ( \
                id INTEGER PRIMARY KEY NOT NULL, \
                email TEXT NOT NULL, \
                token TEXT NOT NULL, \
                created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP \
            )",
        )
        .execute(&self.pool)
        .await?;

        sqlx::query(
            "CREATE TABLE IF NOT EXISTS session_tokens ( \
                id INTEGER PRIMARY KEY NOT NULL, \
                user_id INTEGER NOT NULL, \
                token TEXT NOT NULL, \
                created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP, \
                FOREIGN KEY (user_id) REFERENCES users(id) \
            )",
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    pub async fn create_user(&self, first_name: &str, last_name: &str, email: &str) -> Result<i64> {
        let row = sqlx::query("INSERT INTO users (first_name, last_name, email) VALUES (?, ?, ?)")
            .bind(first_name)
            .bind(last_name)
            .bind(email)
            .execute(&self.pool)
            .await?;
        Ok(row.last_insert_rowid())
    }
    pub async fn lookup_user_by_email(&self, email: &Mailbox) -> Result<Option<User>> {
        let row = sqlx::query_as::<_, User>("SELECT * FROM users WHERE email = ?")
            .bind(email.email.to_string())
            .fetch_optional(&self.pool)
            .await?;
        Ok(row)
    }
    pub async fn lookup_user_by_login_token(&self, token: &str) -> Result<Option<User>> {
        let row = sqlx::query_as::<_, User>(
            "SELECT u.* \
             FROM login_tokens t \
             LEFT JOIN users u on u.email = t.email \
             WHERE t.token = ?",
        )
        .bind(token)
        .fetch_optional(&self.pool)
        .await?;
        Ok(row)
    }
    pub async fn lookup_user_from_session_token(&self, token: &str) -> Result<Option<User>> {
        let user = sqlx::query_as::<_, User>(
            "SELECT u.* \
             FROM session_tokens t \
             JOIN users u on u.id = t.user_id \
             WHERE token = ?",
        )
        .bind(token)
        .fetch_optional(&self.pool)
        .await?;
        Ok(user)
    }

    pub async fn create_session_token(&self, user_id: i64) -> Result<String> {
        let token = format!("{:08x}", OsRng.gen::<u64>());

        sqlx::query("INSERT INTO session_tokens (user_id, token) VALUES (?, ?)")
            .bind(user_id)
            .bind(&token)
            .execute(&self.pool)
            .await?;

        Ok(token)
    }
    pub async fn create_login_token(&self, email: &Mailbox) -> Result<String> {
        let token = format!("{:08x}", OsRng.gen::<u64>());

        sqlx::query("INSERT INTO login_tokens (email, token) VALUES (?, ?)")
            .bind(email.email.to_string())
            .bind(&token)
            .execute(&self.pool)
            .await?;

        Ok(token)
    }
    pub async fn lookup_email_by_login_token(&self, token: &str) -> Result<Option<String>> {
        let row = sqlx::query_as::<_, (String,)>("SELECT email FROM login_tokens WHERE token = ?")
            .bind(token)
            .fetch_optional(&self.pool)
            .await?;
        Ok(row.map(|r| r.0))
    }
}
