//! # 包信息处理模块
//! 
//! 这个模块提供了从镜像站获取和解析包信息的功能。
//! 
//! ## 主要功能
//! 
//! - 下载包信息文件
//! - 解析包信息
//! - 查找特定包
//! 
//! ## 示例
//! 
//! ```rust,no_run
//! use mini_apt::package::package_info::{download_packages_file, parse_packages_file, find_package};
//! 
//! #[tokio::main]
//! async fn main() -> Result<(), String> {
//!     let mirror = "https://mirrors.tuna.tsinghua.edu.cn/ubuntu-ports";
//!     let arch = "arm64";
//! 
//!     // 下载包信息
//!     let content = download_packages_file(mirror, arch).await?;
//! 
//!     // 解析包信息
//!     let packages = parse_packages_file(&content);
//! 
//!     // 查找特定包
//!     if let Some(package) = find_package(&packages, "cpp-x86-64-linux-gnu", arch) {
//!         println!("Found package: {} version {}", package.package, package.version);
//!     }
//!     Ok(())
//! }
//! ```

use std::collections::HashMap;
use std::io::Read;
use reqwest::Client;

use super::PackageInfo;

/// 从镜像站下载包信息文件
/// 
/// # 参数
/// 
/// * `mirror` - 镜像站 URL
/// * `arch` - 目标架构
/// 
/// # 返回值
/// 
/// 成功返回包含包信息的字符串，失败返回错误信息
/// 
/// # 错误
/// 
/// 可能的错误情况：
/// - 网络错误
/// - 解压错误
/// - 无效的响应
pub async fn download_packages_file(mirror: &str, arch: &str) -> Result<String, String> {
    // 尝试不同的仓库组件
    let components = ["main", "universe"];
    let mut all_content = String::new();

    let client = Client::builder()
        .user_agent("Debian APT-HTTP/1.3 (2.0.9)")
        .build()
        .map_err(|e| format!("Failed to create HTTP client: {}", e))?;

    for component in components {
        let url = format!("{}/dists/focal/{}/binary-{}/Packages.gz", mirror, component, arch);
        println!("Trying to download from: {}", url);
        
        match client.get(&url).send().await {
            Ok(response) if response.status().is_success() => {
                match response.bytes().await {
                    Ok(bytes) => {
                        // 解压 gzip 数据
                        let mut decoder = flate2::read::GzDecoder::new(&bytes[..]);
                        let mut content = String::new();
                        if decoder.read_to_string(&mut content).is_ok() {
                            println!("Successfully downloaded {} repository information", component);
                            all_content.push_str(&content);
                            all_content.push('\n');
                        } else {
                            println!("Failed to decompress {} repository information", component);
                        }
                    }
                    Err(e) => println!("Failed to read response from {}: {}", component, e),
                }
            }
            Ok(response) => println!("Failed to download {} repository: {}", component, response.status()),
            Err(e) => println!("Failed to download {} repository: {}", component, e),
        }
    }

    if all_content.is_empty() {
        Err("Failed to download Packages.gz from any component".to_string())
    } else {
        Ok(all_content)
    }
}

/// 解析包信息文件内容
/// 
/// # 参数
/// 
/// * `content` - 包信息文件的内容
/// 
/// # 返回值
/// 
/// 返回包名到包信息的映射
pub fn parse_packages_file(content: &str) -> HashMap<String, PackageInfo> {
    let mut packages = HashMap::new();
    let mut current_package: Option<String> = None;
    let mut current_info: HashMap<String, String> = HashMap::new();

    for line in content.lines() {
        if line.is_empty() {
            // 空行表示一个包的信息结束
            if let Some(package_name) = current_package.take() {
                if let Ok(package_info) = create_package_info(&package_name, &current_info) {
                    packages.insert(package_name, package_info);
                }
            }
            current_info.clear();
            continue;
        }

        if line.starts_with(' ') {
            // 继续上一行的值
            continue;
        }

        if let Some((key, value)) = line.split_once(": ") {
            if key == "Package" {
                current_package = Some(value.to_string());
            }
            current_info.insert(key.to_string(), value.to_string());
        }
    }

    // 处理最后一个包
    if let Some(package_name) = current_package {
        if let Ok(package_info) = create_package_info(&package_name, &current_info) {
            packages.insert(package_name, package_info);
        }
    }

    packages
}

/// 从原始信息创建包信息结构
/// 
/// # 参数
/// 
/// * `package_name` - 包名
/// * `info` - 包含包信息的键值对
/// 
/// # 返回值
/// 
/// 成功返回包信息结构，失败返回错误信息
fn create_package_info(package_name: &str, info: &HashMap<String, String>) -> Result<PackageInfo, String> {
    Ok(PackageInfo::new(
        package_name.to_string(),
        info.get("Version").ok_or("Missing Version")?.to_string(),
        info.get("Architecture").ok_or("Missing Architecture")?.to_string(),
        info.get("Filename").ok_or("Missing Filename")?.to_string(),
        info.get("Size").ok_or("Missing Size")?.parse().map_err(|_| "Invalid Size")?,
        info.get("MD5sum").ok_or("Missing MD5sum")?.to_string(),
        info.get("SHA256").ok_or("Missing SHA256")?.to_string(),
    ))
}

/// 在包集合中查找特定包
/// 
/// # 参数
/// 
/// * `packages` - 包集合
/// * `name` - 要查找的包名
/// * `arch` - 目标架构
/// 
/// # 返回值
/// 
/// 如果找到包则返回 Some(PackageInfo)，否则返回 None
pub fn find_package<'a>(packages: &'a HashMap<String, PackageInfo>, name: &str, arch: &str) -> Option<&'a PackageInfo> {
    packages.get(name).filter(|p| p.architecture == arch)
} 