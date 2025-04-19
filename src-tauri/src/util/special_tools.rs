use std::path::PathBuf;
use crate::util::path;

/// 确定当前是否是开发环境
pub fn is_dev_environment() -> bool {
     // 首先检查环境变量
    cfg!(dev)
}

/// 获取基础存储路径
pub fn get_base_storage_path() -> PathBuf {
    // 不再使用硬编码路径，始终使用应用程序数据目录
    path::paths().app_data_dir()
}

/// 获取特定类型文件的存储路径
pub fn get_storage_path(folder_name: &str) -> PathBuf {
    let base_path = get_base_storage_path();
    base_path.join(folder_name)
}

/// 确保存储路径存在，如果不存在则创建
pub fn ensure_storage_path(folder_name: &str) -> Result<PathBuf, std::io::Error> {
    let path = get_storage_path(folder_name);
    
    if !path.exists() {
        std::fs::create_dir_all(&path)?;
    }
    
    Ok(path)
}

pub fn get_image_path(image_name: &str) -> PathBuf {
    let base_path = get_base_storage_path();
    let image_path = base_path.join("images").join(image_name);
    image_path
}

/// 获取日志文件路径
pub fn get_log_file_path() -> PathBuf {
    let log_dir = ensure_storage_path("logs").expect("无法创建日志目录");
    
    // 使用日期创建日志文件名
    use chrono::Local;
    let today = Local::now().format("%Y-%m-%d").to_string();
    log_dir.join(format!("app-{}.log", today))
}

/// 读取最近的日志
pub fn read_recent_logs(lines: usize) -> Vec<String> {
    let log_path = get_log_file_path();
    
    if !log_path.exists() {
        return vec!["没有找到日志文件".to_string()];
    }
    
    match std::fs::read_to_string(&log_path) {
        Ok(content) => {
            // 获取最后N行
            let all_lines: Vec<&str> = content.lines().collect();
            let start_idx = if all_lines.len() > lines {
                all_lines.len() - lines
            } else {
                0
            };
            
            all_lines[start_idx..].iter().map(|&s| s.to_string()).collect()
        },
        Err(e) => {
            vec![format!("读取日志文件失败: {}", e)]
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_storage_paths() {
        println!("当前环境: {}", if is_dev_environment() { "开发环境" } else { "生产环境" });

        println!("基础存储路径: {}", get_base_storage_path().display());
        println!("日志存储路径: {}", get_storage_path("logs").display());
        println!("缓存存储路径: {}", get_storage_path("cache").display());
    }
}
