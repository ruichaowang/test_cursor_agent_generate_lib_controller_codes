use crate::config::InstallConfig;
use crate::package::package_info::{download_packages_file, parse_packages_file, find_package};
use crate::package::downloader::{download_package, download_packages};

pub struct UrlBuilder;

impl UrlBuilder {
    pub async fn build_package_urls(config: &InstallConfig, mirror: &str) -> bool {
        // 针对不同的包类型构建不同的 URL
        match config.package_name.as_str() {
            name if name.starts_with("android-ndk") => {
                // 使用最新的 NDK 下载链接
                let downloads = vec![
                    ("https://dl.google.com/android/repository/android-ndk-r26b-darwin.dmg".to_string(), config.root_dir.clone(), "dummy".to_string()),
                    ("https://dl.google.com/android/repository/android-ndk-r26b-darwin.zip".to_string(), config.root_dir.clone(), "dummy".to_string()),
                ];
                
                if let Err(e) = download_packages(downloads).await {
                    println!("Failed to download NDK: {}", e);
                    false
                } else {
                    true
                }
            }
            _ => {
                // 从 Packages 文件中获取包信息
                println!("Downloading package information...");
                match download_packages_file(mirror, &config.architecture).await {
                    Ok(packages_content) => {
                        println!("Parsing package information...");
                        let packages = parse_packages_file(&packages_content);
                        println!("Found {} packages", packages.len());
                        
                        // 查找包
                        println!("Looking for package {} with architecture {}", config.package_name, config.architecture);
                        if let Some(package_info) = find_package(&packages, &config.package_name, &config.architecture) {
                            println!("Found package: {} version {}", package_info.package, package_info.version);
                            let url = format!("{}/{}", mirror, package_info.filename);
                            let url_display = url.clone();
                            println!("Trying to download from: {}", url_display);
                            if let Err(e) = download_package(url, config.root_dir.clone(), package_info.md5sum.clone()).await {
                                println!("Package not found at: {} ({})", url_display, e);
                                false
                            } else {
                                println!("Successfully downloaded package from {}", url_display);
                                true
                            }
                        } else {
                            println!("Package not found in repository");
                            false
                        }
                    }
                    Err(e) => {
                        println!("Failed to download package information: {}", e);
                        false
                    }
                }
            }
        }
    }
} 