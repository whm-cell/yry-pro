// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/

pub mod util;

use std::{
    fs,
    path:: PathBuf,
};
use tokio::sync::OnceCell;
use util::{database::{Database, VocabularyRecord, SystemSetting}, special_tools};

// 全局数据库连接
static DB: OnceCell<Database> = OnceCell::const_new();

// 初始化数据库连接
async fn get_db() -> &'static Database {
    DB.get_or_init(|| async { Database::new().await.expect("数据库初始化失败") })
        .await
}

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

// 确保images目录存在
#[tauri::command]
fn ensure_images_dir(_app_handle: tauri::AppHandle) -> Result<String, String> {
    //  let app_dir = util::path::paths().app_local_data_dir().to_string_lossy().into_owned();
    let app_dir = String::from("/Users/coolm/softs/temp_files");
    let images_dir = PathBuf::from(&app_dir).join("images");

    // 确保目录存在
    if !images_dir.exists() {
        fs::create_dir_all(&images_dir).map_err(|e| e.to_string())?;
    }

    Ok(images_dir.to_string_lossy().to_string())
}

// 保存图片到images目录
#[tauri::command]
#[allow(deprecated)]
fn save_image(
    _app_handle: tauri::AppHandle,
    file_data: String,
    file_name: String,
) -> Result<String, String> {
    let app_dir = String::from("/Users/coolm/softs/temp_files");
    let images_dir = PathBuf::from(&app_dir).join("images");

    // 确保目录存在
    if !images_dir.exists() {
        fs::create_dir_all(&images_dir).map_err(|e| e.to_string())?;
    }

    // 解码Base64数据
    let prefix_removed = file_data.split(",").nth(1).unwrap_or(&file_data);
    let image_data = base64::decode(prefix_removed).map_err(|e| e.to_string())?;

    // 构建文件路径
    let file_path = images_dir.join(&file_name);

    // 写入文件
    fs::write(&file_path, image_data).map_err(|e| e.to_string())?;

    // 返回保存的文件路径
    Ok(file_path.to_string_lossy().to_string())
}

// 获取images目录中的图片列表
#[tauri::command]
fn list_images(_app_handle: tauri::AppHandle) -> Result<Vec<String>, String> {
    let app_dir = String::from("/Users/coolm/softs/temp_files");
    let images_dir = PathBuf::from(&app_dir).join("images");

    // 如果目录不存在，创建它
    if !images_dir.exists() {
        fs::create_dir_all(&images_dir).map_err(|e| e.to_string())?;
        return Ok(Vec::new());
    }

    // 读取目录内容
    let mut images = Vec::new();
    for entry in fs::read_dir(images_dir).map_err(|e| e.to_string())? {
        let entry = entry.map_err(|e| e.to_string())?;
        let path = entry.path();
        if path.is_file() {
            let file_name = path
                .file_name()
                .and_then(|name| name.to_str())
                .ok_or_else(|| "Failed to get file name".to_string())?;

            // 只添加图片文件
            if file_name.ends_with(".jpg")
                || file_name.ends_with(".jpeg")
                || file_name.ends_with(".png")
                || file_name.ends_with(".gif")
            {
                images.push(file_name.to_string());
            }
        }
    }

    Ok(images)
}

// 添加单词记录
#[tauri::command]
async fn add_vocabulary(
    word: String,
    translation: String,
    image_path: String,
    color: Option<String>,
) -> Result<i64, String> {
    let db = get_db().await;
    let full_image_path = special_tools::get_image_path(&image_path);
    let record = VocabularyRecord {
        id: None,
        word,
        translation,
        image_path: full_image_path.to_string_lossy().to_string(),
        phonetic: None,
        example: None,
        color,
    };

    db.add_vocabulary(record).await.map_err(|e| e.to_string())
}

// 获取所有单词记录
#[tauri::command]
async fn get_all_vocabulary() -> Result<Vec<VocabularyRecord>, String> {
    let db = get_db().await;

    db.get_all_vocabulary().await.map_err(|e| e.to_string())
}

// 根据ID获取单词记录
#[tauri::command]
async fn get_vocabulary_by_id(id: i64) -> Result<Option<VocabularyRecord>, String> {
    let db = get_db().await;

    db.get_vocabulary_by_id(id).await.map_err(|e| e.to_string())
}

// 删除单词记录
#[tauri::command]
async fn delete_vocabulary(id: i64) -> Result<bool, String> {
    let db = get_db().await;

    db.delete_vocabulary(id).await.map_err(|e| e.to_string())
}

// 获取活动单词列表
#[tauri::command]
async fn get_active_words() -> Result<Vec<VocabularyRecord>, String> {
    let db = get_db().await;

    db.get_active_words().await.map_err(|e| e.to_string())
}

// 添加活动单词
#[tauri::command]
async fn add_active_word(word_id: i64) -> Result<bool, String> {
    let db = get_db().await;

    db.add_active_word(word_id).await.map_err(|e| e.to_string())
}

// 移除活动单词
#[tauri::command]
async fn remove_active_word(word_id: i64) -> Result<bool, String> {
    let db = get_db().await;

    db.remove_active_word(word_id).await.map_err(|e| e.to_string())
}

// 获取系统设置
#[tauri::command]
async fn get_system_setting(key: String) -> Result<Option<SystemSetting>, String> {
    let db = get_db().await;

    db.get_system_setting(&key).await.map_err(|e| e.to_string())
}

// 获取所有系统设置
#[tauri::command]
async fn get_all_system_settings() -> Result<Vec<SystemSetting>, String> {
    let db = get_db().await;

    db.get_all_system_settings().await.map_err(|e| e.to_string())
}

// 更新系统设置
#[tauri::command]
async fn update_system_setting(key: String, value: String) -> Result<bool, String> {
    let db = get_db().await;

    db.update_system_setting(&key, &value).await.map_err(|e| e.to_string())
}

// 更新单词颜色
#[tauri::command]
async fn update_vocabulary_color(id: i64, color: String) -> Result<bool, String> {
    let db = get_db().await;

    db.update_vocabulary_color(id, color).await.map_err(|e| e.to_string())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_log::Builder::new().build())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            greet,
            ensure_images_dir,
            save_image,
            list_images,
            add_vocabulary,
            get_all_vocabulary,
            get_vocabulary_by_id,
            delete_vocabulary,
            get_active_words,
            add_active_word,
            remove_active_word,
            get_system_setting,
            get_all_system_settings,
            update_system_setting,
            update_vocabulary_color,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
