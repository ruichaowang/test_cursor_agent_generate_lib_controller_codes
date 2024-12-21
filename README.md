# Mini-APT

一个用 Rust 编写的简化版 APT 包管理器，支持从 Ubuntu 镜像站异步下载和安装软件包。

## 特性

- 异步并行下载
- MD5 校验和验证
- 支持多镜像源
- 支持 main 和 universe 仓库
- 命令行界面

## 安装

确保你已经安装了 Rust 工具链，然后运行：

```bash
cargo install --path .
```

## 使用方法

### 安装软件包

```bash
mini-apt install -u <mirror-url> -m <architecture> -d <directory> <package-name>
```

例如：
```bash
mini-apt install -u "https://mirrors.tuna.tsinghua.edu.cn/ubuntu-ports" -m arm64 -d sysroot cpp-x86-64-linux-gnu
```

### 参数说明

- `-u, --url <url>`: 镜像源 URL
- `-m, --arch <arch>`: 目标架构（如 arm64, x86_64 等）
- `-d, --dir <dir>`: 安装目录
- `<package>`: 要安装的包名

### 显示帮助

```bash
mini-apt help
```

## 开发

### 依赖

- tokio: 异步运行时
- reqwest: HTTP 客户端
- futures: 异步操作工具
- flate2: gzip 解压
- md5: 校验和验证

### 构建

```bash
cargo build
```

### 测试

```bash
cargo test
```

## 项目结构

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

## 错误处理

程序会处理以下类型的错误：
- 网络错误（下载失败、连接超时等）
- 文件系统错误（权限问题、磁盘空间不足等）
- MD5 校验失败
- 无效的包名或架构
- 无效的目录路径

## 故障排除

### 常见问题

1. 下载失败
   - 检查网络连接
   - 验证镜像源 URL 是否正确
   - 确认包名和架构是否正确

2. MD5 校验失败
   - 重试下载
   - 尝试使用其他镜像源

3. 权限错误
   - 检查目标目录的写入权限
   - 使用适当的权限运行程序

## 许可证

MIT License 