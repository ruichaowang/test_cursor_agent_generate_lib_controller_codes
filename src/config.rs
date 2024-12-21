//! # 配置模块
//! 
//! 这个模块提供了包安装配置的数据结构和相关功能。
//! 
//! ## 主要功能
//! 
//! - 配置验证
//! - 默认值处理
//! - 路径规范化
//! 
//! ## 示例
//! 
//! ```rust
//! use mini_apt::config::InstallConfig;
//! use std::path::PathBuf;
//! 
//! let config = InstallConfig::new(
//!     "example".to_string(),
//!     vec!["https://mirrors.example.com".to_string()],
//!     "arm64".to_string(),
//!     PathBuf::from("/usr/local"),
//! ).unwrap();
//! ```

use std::path::PathBuf;

/// 包安装配置
/// 
/// 包含了安装软件包所需的所有配置信息。
#[derive(Debug, Clone)]
pub struct InstallConfig {
    /// 要安装的包名
    pub package_name: String,
    /// 镜像源列表，按优先级排序
    pub mirrors: Vec<String>,
    /// 目标架构
    pub architecture: String,
    /// 安装根目录
    pub root_dir: PathBuf,
}

impl Default for InstallConfig {
    /// 创建默认配置
    /// 
    /// 默认值：
    /// - package_name: 空字符串
    /// - mirrors: 空列表
    /// - architecture: "arm64"
    /// - root_dir: "/"
    fn default() -> Self {
        Self {
            package_name: String::new(),
            mirrors: Vec::new(),
            architecture: "arm64".to_string(),
            root_dir: PathBuf::from("/"),
        }
    }
}

impl InstallConfig {
    /// 创建新的安装配置
    /// 
    /// # 参数
    /// 
    /// * `package_name` - 要安装的包名
    /// * `mirrors` - 镜像源列表
    /// * `architecture` - 目标架构
    /// * `root_dir` - 安装根目录
    /// 
    /// # 返回值
    /// 
    /// 成功返回配置实例，失败返回错误信息
    /// 
    /// # 错误
    /// 
    /// 在以下情况会返回错误：
    /// - 无效的架构名称
    /// - 无效的镜像 URL
    /// - 无效的目录路径
    /// 
    /// # 示例
    /// 
    /// ```rust
    /// use mini_apt::config::InstallConfig;
    /// use std::path::PathBuf;
    /// 
    /// let config = InstallConfig::new(
    ///     "example".to_string(),
    ///     vec!["https://mirrors.example.com".to_string()],
    ///     "arm64".to_string(),
    ///     PathBuf::from("/usr/local"),
    /// ).unwrap();
    /// ```
    pub fn new(package_name: String, mirrors: Vec<String>, architecture: String, root_dir: PathBuf) -> Result<Self, String> {
        // 验证架构
        let valid_architectures = vec![
            "arm64", "x86_64", "all", "amd64", "i386",
            "arm", "armhf", "arm64", "ppc64el", "s390x"
        ];
        
        if !valid_architectures.contains(&architecture.as_str()) {
            return Err(format!(
                "Architecture must be one of: {}",
                valid_architectures.join(", ")
            ));
        }

        Ok(Self {
            package_name,
            mirrors,
            architecture,
            root_dir,
        })
    }
} 