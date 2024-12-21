use std::path::Path;
use crate::error::{Result, Error};

#[derive(Debug, PartialEq, Clone)]
pub struct InstallConfig {
    pub mirrors: Vec<String>,
    pub architecture: String,
    pub root_dir: String,
    pub package_name: String,
}

impl InstallConfig {
    /// 验证安装配置
    pub fn validate(&self) -> Result<()> {
        // 检查目录是否存在，不存在则创建
        let root_path = Path::new(&self.root_dir);
        if !root_path.exists() {
            std::fs::create_dir_all(root_path)
                .map_err(|e| Error::Io(e))?;
        }

        // 检查目录是否可写
        let test_file = root_path.join(".write_test");
        std::fs::write(&test_file, "test")
            .map_err(|e| Error::Io(e))?;
        std::fs::remove_file(test_file)
            .map_err(|e| Error::Io(e))?;

        Ok(())
    }

    /// 获取包的版本号（可以从配置或环境变量中读取）
    pub fn get_version(&self) -> String {
        // TODO: 实现版本号获取逻辑
        "1.0.0".to_string()
    }

    /// 获取构建号
    pub fn get_build_number(&self) -> String {
        // TODO: 实现构建号获取逻辑
        "1".to_string()
    }
}

impl Default for InstallConfig {
    fn default() -> Self {
        Self {
            mirrors: Vec::new(),
            architecture: String::from("arm64"),  // 默认架构为 arm64
            root_dir: String::from("/"),          // 默认根目录
            package_name: String::new(),
        }
    }
} 