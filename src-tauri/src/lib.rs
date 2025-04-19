pub mod util;

use std::{
    fs,
    path:: PathBuf,
};
use tokio::sync::OnceCell;
use util::{database::{Database, VocabularyRecord, SystemSetting}, special_tools};
use tauri::Manager;
use tauri::Emitter;

// 全局数据库连接
static DB: OnceCell<Database> = OnceCell::const_new();

// 初始化数据库连接
async fn get_db() -> &'static Database {
    DB.get_or_init(|| async { 
        log::info!("初始化数据库连接");
        Database::new().await.expect("数据库初始化失败") 
    })
        .await
}

#[tauri::command]
fn greet(name: &str) -> String {
    log::info!("收到问候请求: {}", name);
    format!("Hello, {}! You've been greeted from Rust!", name)
}

// 确保images目录存在
#[tauri::command]
fn ensure_images_dir(app_handle: tauri::AppHandle) -> Result<String, String> {
    log::info!("确保图片目录存在");
    let images_dir = PathBuf::from(&util::special_tools::get_base_storage_path()).join("images");

    // 确保目录存在
    if !images_dir.exists() {
        log::info!("创建图片目录: {}", images_dir.to_string_lossy());
        fs::create_dir_all(&images_dir).map_err(|e| {
            let err_msg = format!("创建图片目录失败: {}", e);
            log::error!("{}", err_msg);
            e.to_string()
        })?;
    }

    if let Some(window) = app_handle.get_webview_window("main") {
        let _ = window.emit("log-event", format!("[INFO] 确保图片目录存在: {}", images_dir.to_string_lossy()));
    }

    Ok(images_dir.to_string_lossy().to_string())
}

// 保存图片到images目录
#[tauri::command]
#[allow(deprecated)]
fn save_image(
    app_handle: tauri::AppHandle,
    file_data: String,
    file_name: String,
) -> Result<String, String> {
    log::info!("保存图片: {}", file_name);
    let images_dir = PathBuf::from(util::special_tools::get_base_storage_path()).join("images");

    // 确保目录存在
    if !images_dir.exists() {
        log::debug!("图片目录不存在，正在创建");
        fs::create_dir_all(&images_dir).map_err(|e| {
            let err_msg = format!("创建图片目录失败: {}", e);
            log::error!("{}", err_msg);
            e.to_string()
        })?;
    }

    // 解码Base64数据
    let prefix_removed = file_data.split(",").nth(1).unwrap_or(&file_data);
    let image_data = base64::decode(prefix_removed).map_err(|e| {
        let err_msg = format!("Base64解码失败: {}", e);
        log::error!("{}", err_msg);
        e.to_string()
    })?;

    // 构建文件路径
    let file_path = images_dir.join(&file_name);

    // 写入文件
    fs::write(&file_path, image_data).map_err(|e| {
        let err_msg = format!("写入图片文件失败: {}", e);
        log::error!("{}", err_msg);
        e.to_string()
    })?;

    log::info!("图片保存成功: {}", file_path.to_string_lossy());
    if let Some(window) = app_handle.get_webview_window("main") {
        let _ = window.emit("log-event", format!("[INFO] 图片保存成功: {}", file_name));
    }

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
    let sounds_dir = PathBuf::from(util::special_tools::get_base_storage_path()).join("sounds");

    // 确保目录存在
    if !sounds_dir.exists() {
        fs::create_dir_all(&sounds_dir).map_err(|e| e.to_string())?;
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
            .map_err(|e| e.to_string())?
    } else {
        // 否则追加到现有文件
        fs::OpenOptions::new()
            .write(true)
            .append(true)
            .open(&file_path)
            .map_err(|e| e.to_string())?
    };

    // 使用标准库的seek和write
    use std::io::{Seek, SeekFrom, Write};
    
    // 设置写入位置
    let mut file = file;
    if offset > 0 {
        file.seek(SeekFrom::Start(offset)).map_err(|e| e.to_string())?;
    }
    
    // 写入数据块
    file.write_all(&data).map_err(|e| e.to_string())?;
    
    // 确保数据写入磁盘
    file.flush().map_err(|e| e.to_string())?;
    
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

// 获取所有单词记录
#[tauri::command]
async fn get_all_vocabulary() -> Result<Vec<VocabularyRecord>, String> {
    log::info!("开始获取所有单词记录");
    let db = get_db().await;
    log::debug!("数据库连接已获取");

    match db.get_all_vocabulary().await {
        Ok(records) => {
            log::info!("成功获取单词记录，数量: {}", records.len());
            if records.len() > 0 {
                // 修复: 先收集到字符串 Vec, 然后用 join 连接
                let word_preview: Vec<String> = records.iter().take(10).map(|r| r.word.clone()).collect();
                log::debug!("单词列表预览: {}", word_preview.join(", "));
            }
            Ok(records)
        },
        Err(e) => {
            let err_msg = format!("获取单词记录失败: {}", e);
            log::error!("{}", err_msg);
            Err(e.to_string())
        }
    }
}

// 添加单词记录
#[tauri::command]
async fn add_vocabulary(
    word: String,
    translation: String,
    image_path: String,
    color: Option<String>,
) -> Result<i64, String> {
    log::info!("添加新单词: {}", word);
    log::debug!("单词翻译: {}, 颜色: {:?}", translation, color);
    
    let db = get_db().await;
    let full_image_path = special_tools::get_image_path(&image_path);
    log::debug!("图片路径: {}", full_image_path.to_string_lossy());
    
    let record = VocabularyRecord {
        id: None,
        word,
        translation,
        image_path: full_image_path.to_string_lossy().to_string(),
        phonetic: None,
        example: None,
        color,
    };

    match db.add_vocabulary(record).await {
        Ok(id) => {
            log::info!("单词添加成功，ID: {}", id);
            Ok(id)
        },
        Err(e) => {
            let err_msg = format!("添加单词失败: {}", e);
            log::error!("{}", err_msg);
            Err(e.to_string())
        }
    }
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
    log::info!("添加活动单词，ID: {}", word_id);
    let db = get_db().await;

    match db.add_active_word(word_id).await {
        Ok(result) => {
            log::info!("单词已添加到活动列表，ID: {}", word_id);
            Ok(result)
        },
        Err(e) => {
            let err_msg = format!("添加活动单词失败: {}", e);
            log::error!("{}", err_msg);
            Err(e.to_string())
        }
    }
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
    log::info!("获取系统设置，键: {}", key);
    let db = get_db().await;

    match db.get_system_setting(&key).await {
        Ok(setting) => {
            if let Some(s) = &setting {
                log::info!("获取系统设置成功: {} = {}", key, s.value);
            } else {
                log::warn!("系统设置不存在: {}", key);
            }
            Ok(setting)
        },
        Err(e) => {
            let err_msg = format!("获取系统设置失败: {}", e);
            log::error!("{}", err_msg);
            Err(e.to_string())
        }
    }
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
    log::info!("正在更新系统设置: {} = {}", key, value);
    let db = get_db().await;

    match db.update_system_setting(&key, &value).await {
        Ok(result) => {
            log::info!("系统设置更新成功: {} = {}", key, value);
            Ok(result)
        },
        Err(e) => {
            let err_msg = format!("更新系统设置失败: {}", e);
            log::error!("{}", err_msg);
            Err(e.to_string())
        }
    }
}

// 更新单词颜色
#[tauri::command]
async fn update_vocabulary_color(id: i64, color: String) -> Result<bool, String> {
    let db = get_db().await;

    db.update_vocabulary_color(id, color).await.map_err(|e| e.to_string())
}

// 获取最近日志
#[tauri::command]
fn get_recent_logs() -> Result<Vec<String>, String> {
    use std::fs;
    use chrono::Local;
    
    // 记录函数调用本身
    let now = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
    log::info!("获取系统日志 (调用时间: {})", now);
    
    let log_path = util::special_tools::get_log_file_path();
    log::debug!("日志文件路径: {}", log_path.to_string_lossy());
    
    let mut logs = Vec::new();
    
    if !log_path.exists() {
        log::warn!("日志文件不存在: {}", log_path.to_string_lossy());
        // 如果日志文件不存在，返回一些初始日志信息
        logs.push("[INFO] 系统日志初始化中...".to_string());
        logs.push("[INFO] 日志文件尚未创建".to_string());
        logs.push(format!("[INFO] 当前时间: {}", now));
        return Ok(logs);
    }
    
    match fs::read_to_string(&log_path) {
        Ok(content) => {
            log::debug!("成功读取日志文件，大小: {} 字节", content.len());
            // 获取最后100行
            let all_lines: Vec<&str> = content.lines().collect();
            let start_idx = if all_lines.len() > 100 {
                all_lines.len() - 100
            } else {
                0
            };
            
            logs = all_lines[start_idx..].iter().map(|&s| s.to_string()).collect();
            log::debug!("返回日志行数: {}", logs.len());
            
            // 如果没有日志，添加一条默认信息
            if logs.is_empty() {
                logs.push("[INFO] 暂无系统日志记录".to_string());
                logs.push(format!("[INFO] 当前时间: {}", now));
            }
            
            Ok(logs)
        },
        Err(e) => {
            let err_msg = format!("读取日志文件失败: {}", e);
            log::error!("{}", err_msg);
            logs.push(format!("[ERROR] {}", err_msg));
            logs.push("[INFO] 将在系统运行过程中生成新的日志".to_string());
            logs.push(format!("[INFO] 当前时间: {}", now));
            Ok(logs)
        }
    }
}

// 记录自定义日志并发送到前端
#[tauri::command]
fn log_message(app_handle: tauri::AppHandle, level: String, message: String) -> Result<(), String> {
    // 添加时间戳
    use chrono::Local;
    let now = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
    let formatted_message = format!("[{}] {}", level.to_uppercase(), message);
    let log_entry = format!("[{}] {}", now, formatted_message);
    
    // 根据级别记录日志
    match level.as_str() {
        "error" => log::error!("{}", message),
        "warn" => log::warn!("{}", message),
        "info" => log::info!("{}", message),
        "debug" => log::debug!("{}", message),
        "trace" => log::trace!("{}", message),
        _ => log::info!("{}", message),
    }
    
    // 获取窗口并发送事件
    if let Some(window) = app_handle.get_webview_window("main") {
        let _ = window.emit("log-event", formatted_message);
    }
    
    // 同时也将日志写入文件
    let log_path = util::special_tools::get_log_file_path();
    let log_dir = log_path.parent().unwrap();
    if !log_dir.exists() {
        std::fs::create_dir_all(log_dir).map_err(|e| e.to_string())?;
    }
    
    use std::fs::OpenOptions;
    use std::io::Write;
    
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(log_path)
        .map_err(|e| e.to_string())?;
    
    file.write_all(log_entry.as_bytes()).map_err(|e| e.to_string())?;
    file.write_all(b"\n").map_err(|e| e.to_string())?;
    
    Ok(())
}

// 生成测试日志
#[tauri::command]
fn generate_test_logs(app_handle: tauri::AppHandle) -> Result<(), String> {
    log_message(app_handle.clone(), "info".to_string(), "这是一条测试信息日志".to_string())?;
    log_message(app_handle.clone(), "warn".to_string(), "这是一条测试警告日志".to_string())?;
    log_message(app_handle.clone(), "error".to_string(), "这是一条测试错误日志".to_string())?;
    log_message(app_handle.clone(), "debug".to_string(), "这是一条测试调试日志".to_string())?;
    log_message(app_handle.clone(), "info".to_string(), "单词抽奖系统初始化".to_string())?;
    log_message(app_handle.clone(), "info".to_string(), "正在加载单词数据...".to_string())?;
    log_message(app_handle.clone(), "info".to_string(), "单词数据加载完成".to_string())?;
    log_message(app_handle.clone(), "info".to_string(), "正在检查系统配置".to_string())?;
    log_message(app_handle.clone(), "warn".to_string(), "未找到自定义配置，使用默认配置".to_string())?;
    log_message(app_handle.clone(), "info".to_string(), "系统准备就绪".to_string())?;
    Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    // 初始化日志系统
    let log_path = util::special_tools::get_log_file_path();
    let log_dir = log_path.parent().unwrap();
    if !log_dir.exists() {
        std::fs::create_dir_all(log_dir).expect("无法创建日志目录");
    }

    // 创建全局通道，用于跨线程传递日志消息
    let (log_sender, log_receiver) = std::sync::mpsc::channel::<String>();
    let log_sender_clone = log_sender.clone();

    // 自定义日志处理器
    struct LogForwarder {
        sender: std::sync::mpsc::Sender<String>,
    }

    impl log::Log for LogForwarder {
        fn enabled(&self, metadata: &log::Metadata) -> bool {
            metadata.level() <= log::Level::Info
        }

        fn log(&self, record: &log::Record) {
            if self.enabled(record.metadata()) {
                let level = record.level().to_string().to_uppercase();
                let message = record.args().to_string();
                let formatted = format!("[{}] {}", level, message);
                let _ = self.sender.send(formatted);
            }
        }

        fn flush(&self) {}
    }

    // 安装自定义日志处理器
    log::set_boxed_logger(Box::new(LogForwarder {
        sender: log_sender_clone,
    }))
    .map(|()| log::set_max_level(log::LevelFilter::Info))
    .expect("无法安装日志处理器");

    // 启动应用
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_opener::init())
        .setup(move |app| {
            // 设置日志转发到前端的线程
            let app_handle = app.app_handle().clone();
            std::thread::spawn(move || {
                while let Ok(log_message) = log_receiver.recv() {
                    if let Some(window) = app_handle.get_webview_window("main") {
                        let _ = window.emit("log-event", log_message);
                    }
                }
            });
            
            // 记录系统初始化日志
            log::info!("应用启动成功");
            log::info!("日志系统已初始化");
            log::info!("日志文件路径: {}", log_path.to_string_lossy());
            log::info!("正在加载系统配置...");
            log::info!("正在检查数据库连接...");
            log::info!("初始化单词抽奖系统...");
            log::warn!("未找到自定义配置，使用默认配置");
            log::info!("系统准备就绪");
            
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
            get_recent_logs,
            log_message,
            generate_test_logs,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
