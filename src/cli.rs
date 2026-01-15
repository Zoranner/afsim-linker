use crate::config::SCHEME_NAME;
use crate::url_scheme::{is_registered, register_url_scheme, unregister_url_scheme};
use std::env;

#[derive(Debug, Clone)]
pub enum CliCommand {
    Run(Option<String>),
    Unregister,
    Check,
}

pub fn parse_args() -> CliCommand {
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 {
        let scheme_prefix = format!("{}://", SCHEME_NAME);
        match args[1].as_str() {
            "--unregister" => return CliCommand::Unregister,
            "--check" => return CliCommand::Check,
            url if url.starts_with(&scheme_prefix) => {
                return CliCommand::Run(Some(url.to_string()));
            }
            _ => {}
        }
    }

    CliCommand::Run(None)
}

pub fn handle_command(command: CliCommand) -> bool {
    match command {
        CliCommand::Unregister => {
            match unregister_url_scheme() {
                Ok(_) => println!("✓ URL Scheme 卸载成功"),
                Err(e) => {
                    eprintln!("✗ 卸载失败: {}", e);
                    std::process::exit(1);
                }
            }
            false
        }
        CliCommand::Check => {
            match is_registered() {
                Ok(true) => println!("✓ URL Scheme 已注册"),
                Ok(false) => println!("✗ URL Scheme 未注册"),
                Err(e) => {
                    eprintln!("✗ 检查失败: {}", e);
                    std::process::exit(1);
                }
            }
            false
        }
        CliCommand::Run(url) => {
            if let Some(ref u) = url {
                tracing::info!("收到 URL 调用: {}", u);
            }
            true
        }
    }
}

pub fn get_url_from_command(command: &CliCommand) -> Option<String> {
    match command {
        CliCommand::Run(url) => url.clone(),
        _ => None,
    }
}

pub fn ensure_url_scheme_registered() {
    match is_registered() {
        Ok(false) => {
            tracing::info!("URL Scheme 未注册，正在自动注册...");
            if let Err(e) = register_url_scheme() {
                tracing::warn!("自动注册失败: {}，应用将继续运行", e);
            } else {
                tracing::info!("URL Scheme 注册成功");
            }
        }
        Ok(true) => {
            tracing::debug!("URL Scheme 已注册");
        }
        Err(e) => {
            tracing::warn!("检查注册状态失败: {}，应用将继续运行", e);
        }
    }
}
