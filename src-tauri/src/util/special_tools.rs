use std::path::PathBuf;
use crate::util::path;

/// 确定当前是否是开发环境
pub fn is_dev_environment() -> bool {
     // 首先检查环境变量
    cfg!(dev)
}

/// 获取基础存储路径
pub fn get_base_storage_path() -> PathBuf {
    if is_dev_environment() {
        // 开发环境使用固定路径
        PathBuf::from("/Users/coolm/softs/temp_files")
    } else {
        // 生产环境使用 AppData 目录
        path::paths().data_path()
    }
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
