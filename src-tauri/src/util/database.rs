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
    pub color: Option<String>,
    pub is_default: bool,
}

/// 系统设置结构体
#[derive(Debug, Serialize, Deserialize)]
pub struct SystemSetting {
    pub key: String,
    pub value: String,
    pub description: Option<String>,
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

        // 初始化单词表
        sqlx::query(
            "CREATE TABLE IF NOT EXISTS vocabulary (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                word TEXT NOT NULL,
                translation TEXT NOT NULL,
                image_path TEXT NOT NULL,
                phonetic TEXT,
                example TEXT,
                color TEXT,
                is_default BOOLEAN DEFAULT 0,
                created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
            )",
        )
        .execute(&pool)
        .await
        .context("创建单词表失败")?;

        // 检查is_default列是否存在，如果不存在则添加
        let column_exists = sqlx::query("PRAGMA table_info(vocabulary)")
            .map(|row: sqlx::sqlite::SqliteRow| {
                let column_name: String = row.get("name");
                column_name == "is_default"
            })
            .fetch_all(&pool)
            .await
            .context("检查is_default列是否存在失败")?
            .iter()
            .any(|&exists| exists);

        if !column_exists {
            sqlx::query("ALTER TABLE vocabulary ADD COLUMN is_default BOOLEAN DEFAULT 0")
                .execute(&pool)
                .await
                .context("添加is_default列失败")?;
        }

        // 初始化活动单词表
        sqlx::query(
            "CREATE TABLE IF NOT EXISTS active_words (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                word_id INTEGER NOT NULL,
                added_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
                FOREIGN KEY (word_id) REFERENCES vocabulary (id) ON DELETE CASCADE,
                UNIQUE(word_id)
            )",
        )
        .execute(&pool)
        .await
        .context("创建活动单词表失败")?;

        // 初始化系统设置表
        sqlx::query(
            "CREATE TABLE IF NOT EXISTS system_settings (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                key TEXT NOT NULL UNIQUE,
                value TEXT NOT NULL,
                description TEXT,
                updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
            )",
        )
        .execute(&pool)
        .await
        .context("创建系统设置表失败")?;

        // 初始化默认设置
        Self::init_default_settings(&pool).await?;
        
        // 初始化默认单词数据
        Self::init_default_vocabulary(&pool).await?;

        Ok(Self { pool })
    }

    /// 初始化默认设置
    async fn init_default_settings(pool: &SqlitePool) -> Result<()> {
        // 检查max_prize_draws设置是否存在
        let max_prize_draws_exists = sqlx::query("SELECT COUNT(*) as count FROM system_settings WHERE key = 'max_prize_draws'")
            .map(|row: sqlx::sqlite::SqliteRow| {
                row.get::<i64, _>("count") > 0
            })
            .fetch_one(pool)
            .await
            .context("检查max_prize_draws设置是否存在失败")?;

        // 如果max_prize_draws设置不存在，添加默认值
        if !max_prize_draws_exists {
            sqlx::query(
                "INSERT INTO system_settings (key, value, description) VALUES (?, ?, ?)"
            )
            .bind("max_prize_draws")
            .bind("1")
            .bind("普通奖品最多抽取次数")
            .execute(pool)
            .await
            .context("添加默认max_prize_draws设置失败")?;
        }

        Ok(())
    }

    /// 初始化默认单词数据
    async fn init_default_vocabulary(pool: &SqlitePool) -> Result<()> {
        // 检查是否已有单词数据
        let has_vocabulary = sqlx::query("SELECT COUNT(*) as count FROM vocabulary")
            .map(|row: sqlx::sqlite::SqliteRow| {
                row.get::<i64, _>("count") > 0
            })
            .fetch_one(pool)
            .await
            .context("检查单词数据是否存在失败")?;

        // 如果已有数据，不添加默认数据
        if has_vocabulary {
            return Ok(());
        }

        // 添加一些默认单词
        let default_words = [
            ("☆", "礼袋", "lidai.jpg", "#ffbad8"),
        ];

        for (word, translation, image, color) in default_words {
            // 构建图片完整路径
            let image_path = special_tools::get_image_path(image);
            
            sqlx::query(
                "INSERT INTO vocabulary (word, translation, image_path, color, is_default) VALUES (?, ?, ?, ?, ?)"
            )
            .bind(word)
            .bind(translation)
            .bind(image_path.to_string_lossy().to_string())
            .bind(color)
            .bind(true)  // 设置为默认单词
            .execute(pool)
            .await
            .context(format!("添加默认单词失败: {}", word))?;
        }

        // 确保默认单词是活动状态
        let default_word_id = sqlx::query("SELECT id FROM vocabulary WHERE is_default = 1")
            .map(|row: sqlx::sqlite::SqliteRow| row.get::<i64, _>("id"))
            .fetch_one(pool)
            .await
            .context("获取默认单词ID失败")?;

        sqlx::query("INSERT OR REPLACE INTO active_words (word_id) VALUES (?)")
            .bind(default_word_id)
            .execute(pool)
            .await
            .context("设置默认单词为活动状态失败")?;

        Ok(())
    }

    /// 添加单词记录
    pub async fn add_vocabulary(&self, record: VocabularyRecord) -> Result<i64> {
        let result = sqlx::query(
            "INSERT INTO vocabulary (word, translation, image_path, phonetic, example, color, is_default) 
             VALUES (?, ?, ?, ?, ?, ?, ?)",
        )
        .bind(&record.word)
        .bind(&record.translation)
        .bind(&record.image_path)
        .bind(&record.phonetic)
        .bind(&record.example)
        .bind(&record.color)
        .bind(&record.is_default)
        .execute(&self.pool)
        .await
        .context("添加单词记录失败")?;

        Ok(result.last_insert_rowid())
    }

    /// 获取所有单词记录
    pub async fn get_all_vocabulary(&self) -> Result<Vec<VocabularyRecord>> {
        let records = sqlx::query(
            "SELECT id, word, translation, image_path, phonetic, example, color, is_default FROM vocabulary",
        )
        .map(|row: sqlx::sqlite::SqliteRow| VocabularyRecord {
            id: Some(row.get("id")),
            word: row.get("word"),
            translation: row.get("translation"),
            image_path: row.get("image_path"),
            phonetic: row.get("phonetic"),
            example: row.get("example"),
            color: row.get("color"),
            is_default: row.get("is_default"),
        })
        .fetch_all(&self.pool)
        .await
        .context("获取单词记录失败")?;

        Ok(records)
    }

    /// 根据ID获取单词记录
    pub async fn get_vocabulary_by_id(&self, id: i64) -> Result<Option<VocabularyRecord>> {
        let record = sqlx::query(
            "SELECT id, word, translation, image_path, phonetic, example, color, is_default FROM vocabulary WHERE id = ?"
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
                color: row.get("color"),
                is_default: row.get("is_default"),
            }
        })
        .fetch_optional(&self.pool)
        .await
        .context("获取单词记录失败")?;

        Ok(record)
    }

    /// 删除单词记录
    pub async fn delete_vocabulary(&self, id: i64) -> Result<bool> {
        // 检查是否是默认单词
        let is_default = sqlx::query("SELECT is_default FROM vocabulary WHERE id = ?")
            .bind(id)
            .map(|row: sqlx::sqlite::SqliteRow| row.get::<bool, _>("is_default"))
            .fetch_optional(&self.pool)
            .await
            .context("检查是否是默认单词失败")?;

        if let Some(true) = is_default {
            return Err(anyhow::anyhow!("默认单词不能删除"));
        }

        let result = sqlx::query("DELETE FROM vocabulary WHERE id = ?")
            .bind(id)
            .execute(&self.pool)
            .await
            .context("删除单词记录失败")?;

        Ok(result.rows_affected() > 0)
    }

    /// 更新单词颜色
    pub async fn update_vocabulary_color(&self, id: i64, color: String) -> Result<bool> {
        // 检查是否是默认单词
        let is_default = sqlx::query("SELECT is_default FROM vocabulary WHERE id = ?")
            .bind(id)
            .map(|row: sqlx::sqlite::SqliteRow| row.get::<bool, _>("is_default"))
            .fetch_optional(&self.pool)
            .await
            .context("检查是否是默认单词失败")?;

        if let Some(true) = is_default {
            return Err(anyhow::anyhow!("默认单词不能修改颜色"));
        }

        let result = sqlx::query("UPDATE vocabulary SET color = ? WHERE id = ?")
            .bind(color)
            .bind(id)
            .execute(&self.pool)
            .await
            .context("更新单词颜色失败")?;

        Ok(result.rows_affected() > 0)
    }

    /// 更新单词信息
    pub async fn update_vocabulary(&self, id: i64, word: String, translation: String, image_path: String, color: Option<String>) -> Result<bool> {
        // 检查是否是默认单词
        let is_default = sqlx::query("SELECT is_default FROM vocabulary WHERE id = ?")
            .bind(id)
            .map(|row: sqlx::sqlite::SqliteRow| row.get::<bool, _>("is_default"))
            .fetch_optional(&self.pool)
            .await
            .context("检查是否是默认单词失败")?;

        if let Some(true) = is_default {
            return Err(anyhow::anyhow!("默认单词不能修改"));
        }

        let result = sqlx::query(
            "UPDATE vocabulary SET word = ?, translation = ?, image_path = ?, color = ? WHERE id = ?"
        )
        .bind(word)
        .bind(translation)
        .bind(image_path)
        .bind(color)
        .bind(id)
        .execute(&self.pool)
        .await
        .context("更新单词信息失败")?;

        Ok(result.rows_affected() > 0)
    }

    /// 获取所有活动单词
    pub async fn get_active_words(&self) -> Result<Vec<VocabularyRecord>> {
        let records = sqlx::query(
            "SELECT v.id, v.word, v.translation, v.image_path, v.phonetic, v.example, v.color, v.is_default
             FROM vocabulary v
             INNER JOIN active_words a ON v.id = a.word_id
             ORDER BY a.added_at DESC
             LIMIT 5"
        )
        .map(|row: sqlx::sqlite::SqliteRow| VocabularyRecord {
            id: Some(row.get("id")),
            word: row.get("word"),
            translation: row.get("translation"),
            image_path: row.get("image_path"),
            phonetic: row.get("phonetic"),
            example: row.get("example"),
            color: row.get("color"),
            is_default: row.get("is_default"),
        })
        .fetch_all(&self.pool)
        .await
        .context("获取活动单词失败")?;

        Ok(records)
    }

    /// 添加活动单词
    pub async fn add_active_word(&self, word_id: i64) -> Result<bool> {
        // 检查是否是默认单词
        let is_default = sqlx::query("SELECT is_default FROM vocabulary WHERE id = ?")
            .bind(word_id)
            .map(|row: sqlx::sqlite::SqliteRow| row.get::<bool, _>("is_default"))
            .fetch_optional(&self.pool)
            .await
            .context("检查是否是默认单词失败")?;

        if let Some(true) = is_default {
            return Err(anyhow::anyhow!("默认单词已经是活动状态"));
        }

        // 检查活动单词数量，最多允许5个
        let count = sqlx::query("SELECT COUNT(*) as count FROM active_words")
            .map(|row: sqlx::sqlite::SqliteRow| {
                row.get::<i64, _>("count")
            })
            .fetch_one(&self.pool)
            .await
            .context("获取活动单词数量失败")?;

        if count >= 5 {
            return Err(anyhow::anyhow!("活动单词已达到最大限制（5个）"));
        }

        // 添加新的活动单词
        let result = sqlx::query(
            "INSERT OR REPLACE INTO active_words (word_id) VALUES (?)"
        )
        .bind(word_id)
        .execute(&self.pool)
        .await
        .context("添加活动单词失败")?;

        Ok(result.rows_affected() > 0)
    }

    /// 移除活动单词
    pub async fn remove_active_word(&self, word_id: i64) -> Result<bool> {
        // 检查是否是默认单词
        let is_default = sqlx::query("SELECT is_default FROM vocabulary WHERE id = ?")
            .bind(word_id)
            .map(|row: sqlx::sqlite::SqliteRow| row.get::<bool, _>("is_default"))
            .fetch_optional(&self.pool)
            .await
            .context("检查是否是默认单词失败")?;

        if let Some(true) = is_default {
            return Err(anyhow::anyhow!("默认单词不能移除活动状态"));
        }

        let result = sqlx::query("DELETE FROM active_words WHERE word_id = ?")
            .bind(word_id)
            .execute(&self.pool)
            .await
            .context("移除活动单词失败")?;

        Ok(result.rows_affected() > 0)
    }

    /// 获取系统设置
    pub async fn get_system_setting(&self, key: &str) -> Result<Option<SystemSetting>> {
        let setting = sqlx::query(
            "SELECT key, value, description FROM system_settings WHERE key = ?"
        )
        .bind(key)
        .map(|row: sqlx::sqlite::SqliteRow| {
            SystemSetting {
                key: row.get("key"),
                value: row.get("value"),
                description: row.get("description"),
            }
        })
        .fetch_optional(&self.pool)
        .await
        .context("获取系统设置失败")?;

        Ok(setting)
    }

    /// 获取所有系统设置
    pub async fn get_all_system_settings(&self) -> Result<Vec<SystemSetting>> {
        let settings = sqlx::query(
            "SELECT key, value, description FROM system_settings"
        )
        .map(|row: sqlx::sqlite::SqliteRow| {
            SystemSetting {
                key: row.get("key"),
                value: row.get("value"),
                description: row.get("description"),
            }
        })
        .fetch_all(&self.pool)
        .await
        .context("获取所有系统设置失败")?;

        Ok(settings)
    }

    /// 更新系统设置
    pub async fn update_system_setting(&self, key: &str, value: &str) -> Result<bool> {
        let result = sqlx::query(
            "UPDATE system_settings SET value = ?, updated_at = CURRENT_TIMESTAMP WHERE key = ?"
        )
        .bind(value)
        .bind(key)
        .execute(&self.pool)
        .await
        .context("更新系统设置失败")?;

        // 如果未找到设置，尝试插入
        if result.rows_affected() == 0 {
            let result = sqlx::query(
                "INSERT INTO system_settings (key, value) VALUES (?, ?)"
            )
            .bind(key)
            .bind(value)
            .execute(&self.pool)
            .await
            .context("插入系统设置失败")?;

            Ok(result.rows_affected() > 0)
        } else {
            Ok(true)
        }
    }
}
