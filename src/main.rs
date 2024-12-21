#![deny(warnings)]

use std::path::PathBuf;
use std::process;

mod config;
mod package;
mod utils;

use config::InstallConfig;
use utils::url::UrlBuilder;

fn print_usage() {
    println!("Usage: mini-apt <command> [options]");
    println!();
    println!("Commands:");
    println!("  install    Install a package");
    println!("  echo      Echo back the input text");
    println!("  help      Show this help message");
    println!();
    println!("Options for install:");
    println!("  -u, --url <url>          Mirror URL");
    println!("  -m, --arch <arch>        Architecture");
    println!("  -d, --dir <dir>          Root directory");
    println!("  <package>                Package name");
}

#[tokio::main]
async fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        print_usage();
        process::exit(1);
    }

    match args[1].as_str() {
        "install" => {
            let mut i = 2;
            let mut mirror_url = String::new();
            let mut architecture = String::new();
            let mut root_dir = PathBuf::new();
            let mut package_name = String::new();

            while i < args.len() {
                match args[i].as_str() {
                    "-u" | "--url" => {
                        if i + 1 < args.len() {
                            mirror_url = args[i + 1].clone();
                            i += 2;
                        } else {
                            eprintln!("Error: Missing value for --url");
                            process::exit(1);
                        }
                    }
                    "-m" | "--arch" => {
                        if i + 1 < args.len() {
                            architecture = args[i + 1].clone();
                            i += 2;
                        } else {
                            eprintln!("Error: Missing value for --arch");
                            process::exit(1);
                        }
                    }
                    "-d" | "--dir" => {
                        if i + 1 < args.len() {
                            root_dir = PathBuf::from(&args[i + 1]);
                            i += 2;
                        } else {
                            eprintln!("Error: Missing value for --dir");
                            process::exit(1);
                        }
                    }
                    _ => {
                        if package_name.is_empty() {
                            package_name = args[i].clone();
                            i += 1;
                        } else {
                            eprintln!("Error: Unexpected argument: {}", args[i]);
                            process::exit(1);
                        }
                    }
                }
            }

            if mirror_url.is_empty() || architecture.is_empty() || root_dir.as_os_str().is_empty() || package_name.is_empty() {
                eprintln!("Error: Missing required arguments");
                print_usage();
                process::exit(1);
            }

            println!("Installing package with configuration:");
            println!("Package name: {}", package_name);
            println!("Mirrors (in priority order):");
            println!("  0: {}", mirror_url);
            println!("Architecture: {}", architecture);
            println!("Root directory: {}", root_dir.display());

            let config = InstallConfig::new(
                package_name,
                vec![mirror_url.clone()],
                architecture,
                root_dir,
            ).unwrap_or_else(|e| {
                eprintln!("Error: {}", e);
                process::exit(1);
            });

            println!("Downloading package...");
            for mirror in &config.mirrors {
                match UrlBuilder::build_package_urls(&config, mirror).await {
                    true => break,
                    false => {
                        eprintln!("Error: Download error: Failed to download package from mirror {}", mirror);
                        continue;
                    }
                }
            }
        }
        "echo" => {
            if args.len() < 3 {
                eprintln!("Error: Missing text to echo");
                process::exit(1);
            }
            println!("{}", args[2]);
        }
        "help" | "--help" | "-h" => {
            print_usage();
        }
        _ => {
            eprintln!("Error: Unknown command: {}", args[1]);
            print_usage();
            process::exit(1);
        }
    }
}
