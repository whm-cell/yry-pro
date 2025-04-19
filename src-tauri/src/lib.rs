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
    let images_dir = PathBuf::from(&util::special_tools::get_base_storage_path()).join("images");

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
    let base_path = util::special_tools::get_base_storage_path();
    let images_dir = base_path.join("images");

    // 确保目录存在
    if !base_path.exists() {
        fs::create_dir_all(&base_path).map_err(|e| format!("创建基础目录失败: {}", e.to_string()))?;
    }
    
    if !images_dir.exists() {
        fs::create_dir_all(&images_dir).map_err(|e| format!("创建图片目录失败: {}", e.to_string()))?;
    }

    // 测试目录写入权限
    let test_file = images_dir.join(".write_test");
    fs::write(&test_file, b"test").map_err(|e| format!("写入测试失败，可能没有写入权限: {}", e.to_string()))?;
    fs::remove_file(test_file).map_err(|e| format!("删除测试文件失败: {}", e.to_string()))?;

    // 解码Base64数据
    let prefix_removed = file_data.split(",").nth(1).unwrap_or(&file_data);
    let image_data = base64::decode(prefix_removed).map_err(|e| format!("Base64解码失败: {}", e.to_string()))?;

    // 构建文件路径
    let file_path = images_dir.join(&file_name);

    // 写入文件
    fs::write(&file_path, image_data).map_err(|e| format!("写入图片文件失败: {}", e.to_string()))?;

    // 返回保存的文件路径
    Ok(file_path.to_string_lossy().to_string())
}

// 获取images目录中的图片列表
#[tauri::command]
fn list_images(_app_handle: tauri::AppHandle) -> Result<Vec<String>, String> {
    let images_dir = PathBuf::from(&util::special_tools::get_base_storage_path()).join("images");

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

// 确保sounds目录存在
#[tauri::command]
fn ensure_sounds_dir(_app_handle: tauri::AppHandle) -> Result<String, String> {
    let sounds_dir = PathBuf::from(&util::special_tools::get_base_storage_path()).join("sounds");

    // 确保目录存在
    if !sounds_dir.exists() {
        fs::create_dir_all(&sounds_dir).map_err(|e| e.to_string())?;
    }

    Ok(sounds_dir.to_string_lossy().to_string())
}

// 保存音频到sounds目录
#[tauri::command]
#[allow(deprecated)]
fn save_sound(
    _app_handle: tauri::AppHandle,
    file_data: String,
    file_name: String,
) -> Result<String, String> {
    let sounds_dir = PathBuf::from(&util::special_tools::get_base_storage_path()).join("sounds");

    // 确保目录存在
    if !sounds_dir.exists() {
        fs::create_dir_all(&sounds_dir).map_err(|e| e.to_string())?;
    }

    // 解码Base64数据
    let prefix_removed = file_data.split(",").nth(1).unwrap_or(&file_data);
    let sound_data = base64::decode(prefix_removed).map_err(|e| e.to_string())?;

    // 构建文件路径
    let file_path = sounds_dir.join(&file_name);

    // 写入文件
    fs::write(&file_path, sound_data).map_err(|e| e.to_string())?;

    // 返回保存的文件路径
    Ok(file_path.to_string_lossy().to_string())
}

// 获取sounds目录中的音频文件列表
#[tauri::command]
fn list_sounds(_app_handle: tauri::AppHandle) -> Result<Vec<(String, String)>, String> {
    let sounds_dir_path = PathBuf::from(&util::special_tools::get_base_storage_path()).join("sounds");

    // 如果目录不存在，创建它
    if !sounds_dir_path.exists() {
        fs::create_dir_all(&sounds_dir_path).map_err(|e| e.to_string())?;
        return Ok(Vec::new());
    }

    // 读取目录内容
    let mut sounds = Vec::new();
    for entry in fs::read_dir(&sounds_dir_path).map_err(|e| e.to_string())? {
        let entry = entry.map_err(|e| e.to_string())?;
        let path = entry.path();
        if path.is_file() {
            let file_name = path
                .file_name()
                .and_then(|name| name.to_str())
                .ok_or_else(|| "Failed to get file name".to_string())?;

            // 只添加音频文件
            if file_name.ends_with(".mp3")
                || file_name.ends_with(".wav")
                || file_name.ends_with(".ogg")
            {
                let file_path = sounds_dir_path.join(file_name);
                sounds.push((file_name.to_string(), file_path.to_string_lossy().to_string()));
            }
        }
    }

    Ok(sounds)
}

// 保存音频文件（分块上传）
#[tauri::command]
fn save_sound_file(
    _app_handle: tauri::AppHandle,
    name: String,
    data: Vec<u8>,
    offset: u64,
    final_: bool,
) -> Result<String, String> {
    let base_path = util::special_tools::get_base_storage_path();
    let sounds_dir = base_path.join("sounds");

    // 确保基础目录存在
    if !base_path.exists() {
        fs::create_dir_all(&base_path).map_err(|e| format!("创建基础目录失败: {}", e.to_string()))?;
    }

    // 确保声音目录存在
    if !sounds_dir.exists() {
        fs::create_dir_all(&sounds_dir).map_err(|e| format!("创建声音目录失败: {}", e.to_string()))?;
    }

    // 测试目录写入权限
    if offset == 0 {  // 只在第一个块时测试权限
        let test_file = sounds_dir.join(".write_test");
        fs::write(&test_file, b"test").map_err(|e| format!("写入测试失败，可能没有写入权限: {}", e.to_string()))?;
        fs::remove_file(test_file).map_err(|e| format!("删除测试文件失败: {}", e.to_string()))?;
    }
    
    // 构建文件路径
    let file_path = sounds_dir.join(&name);
    
    // 创建或追加到文件
    let file = if offset == 0 {
        // 如果是第一个块，创建新文件
        fs::OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(&file_path)
            .map_err(|e| format!("创建音频文件失败: {}", e.to_string()))?
    } else {
        // 否则追加到现有文件
        fs::OpenOptions::new()
            .write(true)
            .append(true)
            .open(&file_path)
            .map_err(|e| format!("打开音频文件失败: {}", e.to_string()))?
    };

    // 使用标准库的seek和write
    use std::io::{Seek, SeekFrom, Write};
    
    // 设置写入位置
    let mut file = file;
    if offset > 0 {
        file.seek(SeekFrom::Start(offset)).map_err(|e| format!("设置文件位置失败: {}", e.to_string()))?;
    }
    
    // 写入数据块
    file.write_all(&data).map_err(|e| format!("写入音频数据失败: {}", e.to_string()))?;
    
    // 确保数据写入磁盘
    file.flush().map_err(|e| format!("刷新文件缓冲失败: {}", e.to_string()))?;
    
    // 如果是最后一个块，返回完整路径
    if final_ {
        Ok(file_path.to_string_lossy().to_string())
    } else {
        Ok("继续上传".to_string())
    }
}

// 删除音频文件
#[tauri::command]
fn delete_sound_file(
    _app_handle: tauri::AppHandle,
    name: String,
) -> Result<bool, String> {
    let sounds_dir = PathBuf::from(&util::special_tools::get_base_storage_path()).join("sounds");
    let file_path = sounds_dir.join(&name);
    
    // 检查文件是否存在
    if !file_path.exists() {
        return Err(format!("文件不存在: {}", name));
    }
    
    // 删除文件
    fs::remove_file(&file_path).map_err(|e| e.to_string())?;
    
    Ok(true)
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
        is_default: false,
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
    match db.update_vocabulary_color(id, color).await {
        Ok(success) => Ok(success),
        Err(e) => Err(e.to_string()),
    }
}

// 更新单词信息
#[tauri::command]
async fn update_vocabulary(id: i64, word: String, translation: String, image_path: String, color: String) -> Result<bool, String> {
    let db = get_db().await;
    
    // 确保使用完整的图片路径
    let full_image_path = special_tools::get_image_path(&image_path);
    
    match db.update_vocabulary(id, word, translation, full_image_path.to_string_lossy().to_string(), Some(color)).await {
        Ok(success) => Ok(success),
        Err(e) => Err(e.to_string()),
    }
}

// 仅更新单词图片 (允许更新默认单词图片)
#[tauri::command]
async fn update_vocabulary_image(id: i64, image_path: String) -> Result<bool, String> {
    let db = get_db().await;
    
    // 确保使用完整的图片路径
    let full_image_path = special_tools::get_image_path(&image_path);
    
    match db.update_vocabulary_image(id, full_image_path.to_string_lossy().to_string()).await {
        Ok(success) => Ok(success),
        Err(e) => Err(e.to_string()),
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_log::Builder::new().build())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            // 初始化路径
            util::path::init_paths(app)?;
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            greet,
            ensure_images_dir,
            save_image,
            list_images,
            ensure_sounds_dir,
            save_sound,
            save_sound_file,
            delete_sound_file,
            list_sounds,
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
            update_vocabulary,
            update_vocabulary_image,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
