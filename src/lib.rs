//! # Mini-APT
//! 
//! 一个用 Rust 编写的简化版 APT 包管理器。
//! 
//! 这个库提供了从 Ubuntu 镜像站下载和安装软件包的功能，支持异步下载、MD5 校验和多镜像源。
//! 
//! ## 主要功能
//! 
//! - 异步并行下载软件包
//! - MD5 校验和验证
//! - 支持多镜像源
//! - 支持 main 和 universe 仓库
//! 
//! ## 示例
//! 
//! ```rust,no_run
//! use mini_apt::config::InstallConfig;
//! use mini_apt::utils::url::UrlBuilder;
//! use std::path::PathBuf;
//! 
//! #[tokio::main]
//! async fn main() -> Result<(), String> {
//!     let config = InstallConfig::new(
//!         "cpp-x86-64-linux-gnu".to_string(),
//!         vec!["https://mirrors.tuna.tsinghua.edu.cn/ubuntu-ports".to_string()],
//!         "arm64".to_string(),
//!         PathBuf::from("sysroot"),
//!     )?;
//! 
//!     for mirror in &config.mirrors {
//!         if UrlBuilder::build_package_urls(&config, mirror).await {
//!             break;
//!         }
//!     }
//!     Ok(())
//! }
//! ```

pub mod config;
pub mod package;
pub mod utils; 