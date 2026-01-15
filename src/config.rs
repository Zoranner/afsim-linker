use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};

pub const APP_NAME: &str = "AFSim 链接器";
pub const APP_VERSION: &str = env!("CARGO_PKG_VERSION");
pub const SCHEME_NAME: &str = "afsim";
pub const CONFIG_FILE_NAME: &str = "config.json";

#[derive(Debug, Clone)]
pub struct AppConfig {
    pub window_title: String,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            window_title: APP_NAME.to_string(),
        }
    }
}

impl AppConfig {
    pub fn new() -> Self {
        Self::default()
    }
}

/// 执行器配置
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ExecutorConfig {
    /// 可执行程序路径
    #[serde(default)]
    pub executable_path: String,
    /// 脚本基础路径
    #[serde(default)]
    pub script_base_path: String,
}

impl ExecutorConfig {
    /// 加载配置文件
    pub fn load() -> Result<Self> {
        let config_path = Self::get_config_path()?;

        if !config_path.exists() {
            tracing::info!("配置文件不存在，使用默认配置");
            return Ok(Self::default());
        }

        let content = fs::read_to_string(&config_path).context("读取配置文件失败")?;

        let config: Self = serde_json::from_str(&content).context("解析配置文件失败")?;

        tracing::info!("配置文件加载成功: {:?}", config_path);
        Ok(config)
    }

    /// 保存配置文件
    pub fn save(&self) -> Result<()> {
        let config_path = Self::get_config_path()?;

        // 确保配置目录存在
        if let Some(parent) = config_path.parent() {
            fs::create_dir_all(parent).context("创建配置目录失败")?;
        }

        let content = serde_json::to_string_pretty(self).context("序列化配置失败")?;

        fs::write(&config_path, content).context("写入配置文件失败")?;

        tracing::info!("配置文件保存成功: {:?}", config_path);
        Ok(())
    }

    /// 获取配置文件路径
    fn get_config_path() -> Result<PathBuf> {
        let config_dir = if cfg!(windows) {
            // Windows: %APPDATA%\afsim-linker
            std::env::var("APPDATA")
                .map(PathBuf::from)
                .unwrap_or_else(|_| PathBuf::from("."))
                .join("afsim-linker")
        } else {
            // Linux: ~/.config/afsim-linker
            std::env::var("HOME")
                .map(PathBuf::from)
                .unwrap_or_else(|_| PathBuf::from("."))
                .join(".config")
                .join("afsim-linker")
        };

        Ok(config_dir.join(CONFIG_FILE_NAME))
    }

    /// 验证配置是否有效
    pub fn is_valid(&self) -> bool {
        !self.executable_path.is_empty()
            && !self.script_base_path.is_empty()
            && Path::new(&self.executable_path).exists()
            && Path::new(&self.script_base_path).exists()
    }

    /// 构建脚本完整路径
    pub fn build_script_path(&self, scene_name: &str, script_entry: &str) -> PathBuf {
        Path::new(&self.script_base_path)
            .join(scene_name)
            .join(script_entry)
    }
}
