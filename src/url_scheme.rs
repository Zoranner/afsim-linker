use crate::config::{APP_NAME, SCHEME_NAME};
use anyhow::{Context, Result};
use std::env;

#[cfg(windows)]
use winreg::enums::*;
#[cfg(windows)]
use winreg::RegKey;

#[cfg(target_os = "linux")]
use std::fs;
#[cfg(target_os = "linux")]
use std::path::PathBuf;

#[cfg(windows)]
pub fn register_url_scheme() -> Result<()> {
    let exe_path = env::current_exe().context("无法获取当前可执行文件路径")?;
    let exe_path_str = exe_path.to_string_lossy().to_string();

    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    let path = format!("Software\\Classes\\{}", SCHEME_NAME);
    let (key, _) = hkcu.create_subkey(&path)?;

    key.set_value("", &format!("URL:{} Protocol", APP_NAME))?;
    key.set_value("URL Protocol", &"")?;

    let (icon_key, _) = key.create_subkey("DefaultIcon")?;
    icon_key.set_value("", &format!("\"{}\",0", exe_path_str))?;

    let (command_key, _) = key.create_subkey("shell\\open\\command")?;
    command_key.set_value("", &format!("\"{}\" \"%1\"", exe_path_str))?;

    tracing::info!("URL Scheme '{}' 注册成功", SCHEME_NAME);
    Ok(())
}

#[cfg(windows)]
pub fn unregister_url_scheme() -> Result<()> {
    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    let path = format!("Software\\Classes\\{}", SCHEME_NAME);

    match hkcu.delete_subkey_all(&path) {
        Ok(_) => {
            tracing::info!("URL Scheme '{}' 卸载成功", SCHEME_NAME);
            Ok(())
        }
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
            tracing::warn!("URL Scheme '{}' 不存在", SCHEME_NAME);
            Ok(())
        }
        Err(e) => Err(e.into()),
    }
}

#[cfg(windows)]
pub fn is_registered() -> Result<bool> {
    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    let path = format!("Software\\Classes\\{}", SCHEME_NAME);

    match hkcu.open_subkey(&path) {
        Ok(_) => Ok(true),
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(false),
        Err(e) => Err(e.into()),
    }
}

#[cfg(windows)]
pub fn get_registry_path() -> String {
    format!("HKEY_CURRENT_USER\\Software\\Classes\\{}", SCHEME_NAME)
}

#[cfg(target_os = "linux")]
pub fn register_url_scheme() -> Result<()> {
    let exe_path = env::current_exe().context("无法获取当前可执行文件路径")?;
    let exe_path_str = exe_path.to_string_lossy();

    let home = env::var("HOME").context("无法获取 HOME 环境变量")?;
    let desktop_file_path = PathBuf::from(&home)
        .join(".local/share/applications")
        .join(format!("{}.desktop", SCHEME_NAME));

    if let Some(parent) = desktop_file_path.parent() {
        fs::create_dir_all(parent)?;
    }

    let desktop_content = format!(
        "[Desktop Entry]\n\
         Version=1.0\n\
         Type=Application\n\
         Name={}\n\
         Comment=Client Linker Application\n\
         Exec={} %u\n\
         Icon={}\n\
         Terminal=false\n\
         Categories=Utility;\n\
         MimeType=x-scheme-handler/{};\n\
         StartupNotify=true\n",
        APP_NAME, exe_path_str, exe_path_str, SCHEME_NAME
    );

    fs::write(&desktop_file_path, desktop_content)?;

    let mimeapps_path = PathBuf::from(&home).join(".local/share/applications/mimeapps.list");

    let mime_type = format!("x-scheme-handler/{}", SCHEME_NAME);
    let desktop_file_name = format!("{}.desktop", SCHEME_NAME);

    let mut mimeapps_content = if mimeapps_path.exists() {
        fs::read_to_string(&mimeapps_path)?
    } else {
        String::from("[Default Applications]\n")
    };

    if !mimeapps_content.contains(&mime_type) {
        if !mimeapps_content.contains("[Default Applications]") {
            mimeapps_content.push_str("\n[Default Applications]\n");
        }

        if !mimeapps_content.ends_with('\n') {
            mimeapps_content.push('\n');
        }

        mimeapps_content.push_str(&format!("{}={}\n", mime_type, desktop_file_name));
        fs::write(&mimeapps_path, mimeapps_content)?;
    }

    std::process::Command::new("update-desktop-database")
        .arg(PathBuf::from(&home).join(".local/share/applications"))
        .output()
        .ok();

    tracing::info!("URL Scheme '{}' 注册成功 (Linux)", SCHEME_NAME);
    Ok(())
}

#[cfg(target_os = "linux")]
pub fn unregister_url_scheme() -> Result<()> {
    let home = env::var("HOME").context("无法获取 HOME 环境变量")?;
    let desktop_file_path = PathBuf::from(&home)
        .join(".local/share/applications")
        .join(format!("{}.desktop", SCHEME_NAME));

    if desktop_file_path.exists() {
        fs::remove_file(&desktop_file_path)?;
    }

    let mimeapps_path = PathBuf::from(&home).join(".local/share/applications/mimeapps.list");

    if mimeapps_path.exists() {
        let mime_type = format!("x-scheme-handler/{}", SCHEME_NAME);
        let content = fs::read_to_string(&mimeapps_path)?;
        let new_content: String = content
            .lines()
            .filter(|line| !line.contains(&mime_type))
            .collect::<Vec<_>>()
            .join("\n");

        if new_content != content {
            fs::write(&mimeapps_path, new_content + "\n")?;
        }
    }

    std::process::Command::new("update-desktop-database")
        .arg(PathBuf::from(&home).join(".local/share/applications"))
        .output()
        .ok();

    tracing::info!("URL Scheme '{}' 卸载成功 (Linux)", SCHEME_NAME);
    Ok(())
}

#[cfg(target_os = "linux")]
pub fn is_registered() -> Result<bool> {
    let home = env::var("HOME").context("无法获取 HOME 环境变量")?;
    let desktop_file_path = PathBuf::from(&home)
        .join(".local/share/applications")
        .join(format!("{}.desktop", SCHEME_NAME));

    Ok(desktop_file_path.exists())
}

#[cfg(target_os = "linux")]
pub fn get_registry_path() -> String {
    format!("~/.local/share/applications/{}.desktop", SCHEME_NAME)
}

#[cfg(not(any(windows, target_os = "linux")))]
pub fn register_url_scheme() -> Result<()> {
    anyhow::bail!("当前平台暂不支持 URL Scheme 注册")
}

#[cfg(not(any(windows, target_os = "linux")))]
pub fn unregister_url_scheme() -> Result<()> {
    anyhow::bail!("当前平台暂不支持 URL Scheme 卸载")
}

#[cfg(not(any(windows, target_os = "linux")))]
pub fn is_registered() -> Result<bool> {
    Ok(false)
}

#[cfg(not(any(windows, target_os = "linux")))]
pub fn get_registry_path() -> String {
    String::from("不支持的平台")
}

pub fn parse_url_scheme(url: &str) -> Option<UrlSchemeData> {
    if !url.starts_with(&format!("{}://", SCHEME_NAME)) {
        return None;
    }

    let content = url.trim_start_matches(&format!("{}://", SCHEME_NAME));

    let (action, params) = if let Some(pos) = content.find('?') {
        let action = &content[..pos];
        let params = &content[pos + 1..];
        (action, Some(params))
    } else {
        (content, None)
    };

    // 解析参数
    let mut scene_name = None;
    let mut script_entry = None;
    let mut parsed_params = std::collections::HashMap::new();

    if let Some(param_str) = params {
        for pair in param_str.split('&') {
            if let Some(pos) = pair.find('=') {
                let key = urlencoding::decode(&pair[..pos]).ok()?;
                let value = urlencoding::decode(&pair[pos + 1..]).ok()?;

                match key.as_ref() {
                    "scene" => scene_name = Some(value.to_string()),
                    "script" => script_entry = Some(value.to_string()),
                    _ => {
                        parsed_params.insert(key.to_string(), value.to_string());
                    }
                }
            }
        }
    }

    Some(UrlSchemeData {
        action: action.to_string(),
        scene_name,
        script_entry,
        params: params.map(|p| p.to_string()),
        parsed_params,
    })
}

#[derive(Debug, Clone)]
pub struct UrlSchemeData {
    pub action: String,
    pub scene_name: Option<String>,
    pub script_entry: Option<String>,
    pub params: Option<String>,
    pub parsed_params: std::collections::HashMap<String, String>,
}
