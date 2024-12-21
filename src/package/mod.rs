//! # 包管理模块
//! 
//! 这个模块提供了软件包的基本数据结构和管理功能。
//! 
//! ## 主要组件
//! 
//! - `PackageInfo`: 软件包信息结构
//! - `downloader`: 包下载功能
//! - `package_info`: 包信息处理功能
//! 
//! ## 示例
//! 
//! ```rust,no_run
//! use mini_apt::package::PackageInfo;
//! 
//! let package = PackageInfo::new(
//!     "example".to_string(),
//!     "1.0.0".to_string(),
//!     "arm64".to_string(),
//!     "pool/main/e/example/example_1.0.0_arm64.deb".to_string(),
//!     1024,
//!     "abcdef1234567890".to_string(),
//!     "0123456789abcdef".to_string(),
//! );
//! ```

pub mod downloader;
pub mod package_info;

#[derive(Debug, Clone)]
#[allow(dead_code)]
/// 软件包信息结构
/// 
/// 包含了一个软件包的所有必要信息，包括名称、版本、架构等。
pub struct PackageInfo {
    /// 包名
    pub package: String,
    /// 版本号
    pub version: String,
    /// 目标架构
    pub architecture: String,
    /// 文件路径
    pub filename: String,
    /// 文件大小（字节）
    pub size: u64,
    /// MD5 校验和
    pub md5sum: String,
    /// SHA256 校验和
    pub sha256: String,
}

impl PackageInfo {
    /// 创建一个新的包信息实例
    /// 
    /// # 参数
    /// 
    /// * `package` - 包名
    /// * `version` - 版本号
    /// * `architecture` - 目标架构
    /// * `filename` - 文件路径
    /// * `size` - 文件大小
    /// * `md5sum` - MD5 校验和
    /// * `sha256` - SHA256 校验和
    /// 
    /// # 示例
    /// 
    /// ```rust
    /// use mini_apt::package::PackageInfo;
    /// 
    /// let package = PackageInfo::new(
    ///     "example".to_string(),
    ///     "1.0.0".to_string(),
    ///     "arm64".to_string(),
    ///     "pool/main/e/example/example_1.0.0_arm64.deb".to_string(),
    ///     1024,
    ///     "abcdef1234567890".to_string(),
    ///     "0123456789abcdef".to_string(),
    /// );
    /// ```
    pub fn new(package: String, version: String, architecture: String, filename: String, size: u64, md5sum: String, sha256: String) -> Self {
        Self {
            package,
            version,
            architecture,
            filename,
            size,
            md5sum,
            sha256,
        }
    }
}
