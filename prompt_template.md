# Mini-APT 项目 Prompt 模板

## 项目概述

请帮我创建一个名为 mini-apt 的命令行工具，这是一个简化版的 APT 包管理器。该工具应该能够从 Ubuntu 镜像站下载和安装软件包，具有异步下载、MD5 校验和多镜像源支持等功能。

## 技术要求

### 1. 基础架构
- 使用 Rust 2021 edition
- 采用异步编程模型
- 支持并行下载
- 模块化设计

### 2. 依赖要求
- tokio 异步运行时
- reqwest HTTP 客户端
- futures 用于异步操作
- flate2 用于 gzip 解压
- md5 用于校验和验证

### 3. 功能规范

#### 命令行接口
- 支持以下命令：
  - install：安装软件包
  - echo：回显测试
  - help：显示帮助信息
- install 命令参数：
  ```
  -u, --url <url>          镜像源 URL
  -m, --arch <arch>        目标架构
  -d, --dir <dir>          安装目录
  <package>                包名
  ```

#### 包管理功能
1. 包信息处理
   - 从镜像站下载 Packages.gz
   - 解析包信息（版本、架构、依赖等）
   - 支持 main 和 universe 仓库

2. 下载功能
   - 异步并行下载
   - MD5 校验
   - 自动创建目标目录
   - 支持断点续传（可选）

3. 错误处理
   - 详细的错误信息
   - 网络错误恢复
   - 文件系统错误处理
   - 用户友好的错误提示

### 4. 项目结构
```
src/
  ├── main.rs          # 程序入口
  ├── config.rs        # 配置处理
  ├── package/         # 包管理模块
  │   ├── mod.rs      # 包定义
  │   ├── downloader.rs # 下载器
  │   └── package_info.rs # 包信息
  └── utils/
      └── url.rs       # URL 处理
```

### 5. 具体实现要求

#### 包信息结构
```rust
pub struct PackageInfo {
    pub package: String,      // 包名
    pub version: String,      // 版本
    pub architecture: String, // 架构
    pub filename: String,     // 文件名
    pub size: u64,           // 大小
    pub md5sum: String,      // MD5校验和
    pub sha256: String,      // SHA256校验和
}
```

#### 异步下载接口
```rust
pub async fn download_package(url: String, root_dir: PathBuf, expected_md5: String) -> Result<(), String>;
pub async fn download_packages(downloads: Vec<(String, PathBuf, String)>) -> Result<(), String>;
```

#### 包信息处理接口
```rust
pub async fn download_packages_file(mirror: &str, arch: &str) -> Result<String, String>;
pub fn parse_packages_file(content: &str) -> HashMap<String, PackageInfo>;
```

### 6. 安全要求
- 所有用户输入必须经过验证
- 下载前验证包的存在性
- 下载后进行 MD5 校验
- 安全的文件系统操作
- 防止路径遍历攻击

### 7. 性能要求
- 支持并行下载多个包
- 异步 I/O 操作
- 高效的包信息解析
- 内存使用优化

### 8. 用户体验
- 清晰的命令行帮助信息
- 下载进度显示
- 详细的错误提示
- 操作确认提示

## 使用示例

```bash
# 安装包
mini-apt install -u "https://mirrors.tuna.tsinghua.edu.cn/ubuntu-ports" -m arm64 -d sysroot cpp-x86-64-linux-gnu

# 显示帮助
mini-apt help
```

## 预期输出示例

```
Installing package with configuration:
Package name: cpp-x86-64-linux-gnu
Mirrors (in priority order):
  0: https://mirrors.tuna.tsinghua.edu.cn/ubuntu-ports
Architecture: arm64
Root directory: sysroot
Downloading package...
MD5 checksum verified successfully
Successfully downloaded package
```

## 特殊处理要求

1. 特定包处理
   - android-ndk 包使用特殊的下载链接
   - 支持不同的仓库组件（main, universe）

2. HTTP 请求
   - 使用标准 Debian APT User-Agent
   - 支持 HTTP/HTTPS
   - 处理重定向

3. 文件系统操作
   - 支持相对和绝对路径
   - 自动创建目录结构
   - 安全的文件写入

## 测试要求

1. 单元测试
   - 包信息解析测试
   - MD5 校验测试
   - 错误处理测试

2. 集成测试
   - 完整的下载流程测试
   - 命令行参数测试
   - 错误恢复测试

## 文档要求

1. 代码注释
   - 函数文档
   - 关键算法说明
   - 错误处理说明

2. 使用文档
   - 安装说明
   - 使用示例
   - 故障排除指南 