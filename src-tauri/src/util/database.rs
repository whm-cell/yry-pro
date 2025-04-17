use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use sqlx::{migrate::MigrateDatabase, Row, Sqlite, SqlitePool};
use crate::util::special_tools;


// 获取数据库URL
fn get_db_url() -> String {
    let path = special_tools::get_base_storage_path();
    format!("sqlite:///{}/vocabulary.db", path.display())
}


/// 单词记录结构体
#[derive(Debug, Serialize, Deserialize)]
pub struct VocabularyRecord {
    pub id: Option<i64>,
    pub word: String,
    pub translation: String,
    pub image_path: String,
    pub phonetic: Option<String>,
    pub example: Option<String>,
}

/// 数据库接口
pub struct Database {
    pool: SqlitePool,
}

impl Database {
    /// 创建或连接到数据库
    pub async fn new() -> Result<Self> {
        let db_url = get_db_url();
        
        // 确保数据库存在
        if !Sqlite::database_exists(&db_url).await.unwrap_or(false) {
            Sqlite::create_database(&db_url)
                .await
                .context("创建数据库失败")?;
        }

        // 连接到数据库
        let pool = SqlitePool::connect(&db_url)
            .await
            .context("连接数据库失败")?;

        // 初始化表
        sqlx::query(
            "CREATE TABLE IF NOT EXISTS vocabulary (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                word TEXT NOT NULL,
                translation TEXT NOT NULL,
                image_path TEXT NOT NULL,
                phonetic TEXT,
                example TEXT,
                created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
            )",
        )
        .execute(&pool)
        .await
        .context("创建表失败")?;

        Ok(Self { pool })
    }

    /// 添加单词记录
    pub async fn add_vocabulary(&self, record: VocabularyRecord) -> Result<i64> {
        let result = sqlx::query(
            "INSERT INTO vocabulary (word, translation, image_path, phonetic, example) 
             VALUES (?, ?, ?, ?, ?)",
        )
        .bind(&record.word)
        .bind(&record.translation)
        .bind(&record.image_path)
        .bind(&record.phonetic)
        .bind(&record.example)
        .execute(&self.pool)
        .await
        .context("添加单词记录失败")?;

        Ok(result.last_insert_rowid())
    }

    /// 获取所有单词记录
    pub async fn get_all_vocabulary(&self) -> Result<Vec<VocabularyRecord>> {
        let records = sqlx::query(
            "SELECT id, word, translation, image_path, phonetic, example FROM vocabulary",
        )
        .map(|row: sqlx::sqlite::SqliteRow| VocabularyRecord {
            id: Some(row.get("id")),
            word: row.get("word"),
            translation: row.get("translation"),
            image_path: row.get("image_path"),
            phonetic: row.get("phonetic"),
            example: row.get("example"),
        })
        .fetch_all(&self.pool)
        .await
        .context("获取单词记录失败")?;
        dbg!(&records);
        Ok(records)
    }

    /// 根据ID获取单词记录
    pub async fn get_vocabulary_by_id(&self, id: i64) -> Result<Option<VocabularyRecord>> {
        let record = sqlx::query(
            "SELECT id, word, translation, image_path, phonetic, example FROM vocabulary WHERE id = ?"
        )
        .bind(id)
        .map(|row: sqlx::sqlite::SqliteRow| {
            VocabularyRecord {
                id: Some(row.get("id")),
                word: row.get("word"),
                translation: row.get("translation"),
                image_path: row.get("image_path"),
                phonetic: row.get("phonetic"),
                example: row.get("example"),
            }
        })
        .fetch_optional(&self.pool)
        .await
        .context("获取单词记录失败")?;

        Ok(record)
    }

    /// 删除单词记录
    pub async fn delete_vocabulary(&self, id: i64) -> Result<bool> {
        let result = sqlx::query("DELETE FROM vocabulary WHERE id = ?")
            .bind(id)
            .execute(&self.pool)
            .await
            .context("删除单词记录失败")?;

        Ok(result.rows_affected() > 0)
    }
}
