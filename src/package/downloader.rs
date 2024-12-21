//! # 包下载器模块
//! 
//! 这个模块提供了异步下载功能，支持单个包下载和并行多包下载。
//! 
//! ## 主要功能
//! 
//! - 异步下载单个包
//! - 并行下载多个包
//! - MD5 校验
//! - 自动创建目录
//! 
//! ## 示例
//! 
//! ```rust,no_run
//! use mini_apt::package::downloader::{download_package, download_packages};
//! use std::path::PathBuf;
//! 
//! #[tokio::main]
//! async fn main() -> Result<(), String> {
//!     // 下载单个包
//!     download_package(
//!         "https://example.com/package.deb".to_string(),
//!         PathBuf::from("downloads"),
//!         "abcdef1234567890".to_string(),
//!     ).await?;
//! 
//!     // 并行下载多个包
//!     let downloads = vec![
//!         ("https://example.com/package1.deb".to_string(),
//!          PathBuf::from("downloads"),
//!          "abcdef1234567890".to_string()),
//!         ("https://example.com/package2.deb".to_string(),
//!          PathBuf::from("downloads"),
//!          "0123456789abcdef".to_string()),
//!     ];
//!     download_packages(downloads).await?;
//!     Ok(())
//! }
//! ```

use std::path::PathBuf;
use reqwest::Client;
use std::fs;
use tokio::io::AsyncWriteExt;

/// 异步下载单个包
/// 
/// # 参数
/// 
/// * `url` - 包的下载 URL
/// * `root_dir` - 下载目标目录
/// * `expected_md5` - 预期的 MD5 校验和
/// 
/// # 返回值
/// 
/// 成功返回 `Ok(())`，失败返回包含错误信息的 `Err(String)`
/// 
/// # 错误
/// 
/// 可能的错误情况：
/// - 网络错误
/// - 文件系统错误
/// - MD5 校验失败
pub async fn download_package(url: String, root_dir: PathBuf, expected_md5: String) -> Result<(), String> {
    // 获取当前工作目录
    let current_dir = std::env::current_dir()
        .map_err(|e| format!("Failed to get current directory: {}", e))?;
    
    // 使用绝对路径创建目标目录
    let absolute_root_dir = if root_dir.is_absolute() {
        root_dir
    } else {
        current_dir.join(root_dir)
    };

    fs::create_dir_all(&absolute_root_dir)
        .map_err(|e| format!("Failed to create directory: {}", e))?;

    let client = Client::builder()
        .user_agent("Debian APT-HTTP/1.3 (2.0.9)")
        .build()
        .map_err(|e| format!("Failed to create HTTP client: {}", e))?;

    let response = client.get(&url)
        .send()
        .await
        .map_err(|e| format!("Failed to send request: {}", e))?;

    if !response.status().is_success() {
        return Err(format!("Status: {} {}", response.status(), response.status().canonical_reason().unwrap_or("")));
    }

    let package_name = url.split('/').last()
        .ok_or_else(|| "Invalid URL".to_string())?;
    let package_path = absolute_root_dir.join(package_name);

    let content = response.bytes()
        .await
        .map_err(|e| format!("Failed to get response content: {}", e))?;

    // 计算下载内容的 MD5
    let actual_md5 = format!("{:x}", md5::compute(&content));

    // 验证 MD5
    if actual_md5 != expected_md5 {
        return Err(format!("MD5 checksum mismatch. Expected: {}, got: {}", expected_md5, actual_md5));
    }

    // 异步写入文件
    let mut file = tokio::fs::File::create(&package_path)
        .await
        .map_err(|e| format!("Failed to create file: {}", e))?;
    file.write_all(&content)
        .await
        .map_err(|e| format!("Failed to write file: {}", e))?;

    println!("MD5 checksum verified successfully");
    Ok(())
}

/// 并行下载多个包
/// 
/// # 参数
/// 
/// * `downloads` - 包含 (URL, 目标目录, MD5) 元组的向量
/// 
/// # 返回值
/// 
/// 成功返回 `Ok(())`，失败返回包含错误信息的 `Err(String)`
/// 
/// # 错误
/// 
/// 如果任何一个包下载失败，整个操作都会失败
pub async fn download_packages(downloads: Vec<(String, PathBuf, String)>) -> Result<(), String> {
    let futures = downloads.into_iter().map(|(url, root_dir, md5)| {
        download_package(url, root_dir, md5)
    });

    futures::future::try_join_all(futures)
        .await
        .map(|_| ())
        .map_err(|e| format!("Failed to download packages: {}", e))
} 