use crate::config::ExecutorConfig;
use crate::executor::{self, ExecutionResult};
use crate::url_scheme::{
    is_registered, parse_url_scheme, register_url_scheme, unregister_url_scheme, UrlSchemeData,
};
use std::time::{Duration, Instant};

pub struct ClientLinkerApp {
    url_data: Option<UrlSchemeData>,
    status_message: Option<String>,
    is_registered: bool,
    executor_config: ExecutorConfig,
    last_execution: Option<ExecutionResult>,
    is_url_mode: bool,
    has_executed: bool,
    execution_time: Option<Instant>,
    min_display_duration: Duration,
}

impl ClientLinkerApp {
    pub fn new(url: Option<String>) -> Self {
        // 加载执行器配置
        let executor_config = ExecutorConfig::load().unwrap_or_default();

        tracing::info!("收到的 URL 参数: {:?}", url);

        let url_data = url.and_then(|u| {
            if u.starts_with("afsim://") {
                tracing::info!("解析 afsim:// URL: {}", u);
                let parsed = parse_url_scheme(&u);
                tracing::info!("解析结果: {:?}", parsed);
                parsed
            } else {
                tracing::warn!("URL 不是 afsim:// 格式: {}", u);
                None
            }
        });

        let is_registered = is_registered().unwrap_or(false);

        if is_registered {
            tracing::info!("URL Scheme 已注册");
        } else {
            tracing::info!("URL Scheme 未注册");
        }

        // 判断是否为 URL 模式（通过 URL 拉起）
        let is_url_mode = url_data.is_some();

        tracing::info!("is_url_mode: {}", is_url_mode);

        Self {
            url_data,
            status_message: None,
            is_registered,
            executor_config,
            last_execution: None,
            is_url_mode,
            has_executed: false,
            execution_time: None,
            min_display_duration: Duration::from_millis(800),
        }
    }

    pub fn handle_register(&mut self) {
        match register_url_scheme() {
            Ok(_) => {
                tracing::info!("协议注册成功");
                self.is_registered = true;
                self.status_message = None;
            }
            Err(e) => {
                tracing::error!("注册失败: {}", e);
                self.status_message = Some(format!("注册失败: {}", e));
            }
        }
    }

    pub fn handle_unregister(&mut self) {
        match unregister_url_scheme() {
            Ok(_) => {
                tracing::info!("协议卸载成功");
                self.is_registered = false;
                self.status_message = None;
            }
            Err(e) => {
                tracing::error!("卸载失败: {}", e);
                self.status_message = Some(format!("卸载失败: {}", e));
            }
        }
    }

    pub fn save_config(&mut self) -> Result<(), String> {
        self.executor_config
            .save()
            .map_err(|e| format!("保存配置失败: {}", e))?;
        self.status_message = Some("配置已保存".to_string());
        Ok(())
    }

    pub fn save_config_with_validation(&mut self) {
        // 验证路径
        if self.executor_config.executable_path.is_empty() {
            self.status_message = Some("可执行程序路径不能为空".to_string());
            return;
        }

        if self.executor_config.script_base_path.is_empty() {
            self.status_message = Some("脚本基础路径不能为空".to_string());
            return;
        }

        if !executor::validate_path(&self.executor_config.executable_path) {
            self.status_message = Some("可执行程序路径无效或文件不存在".to_string());
            return;
        }

        if !executor::validate_path(&self.executor_config.script_base_path) {
            self.status_message = Some("脚本基础路径无效或目录不存在".to_string());
            return;
        }

        // 保存配置
        if let Err(e) = self.save_config() {
            self.status_message = Some(e);
        }
    }

    pub fn execute_from_url(&mut self) {
        self.has_executed = true;
        self.execution_time = Some(Instant::now());

        if let Some(ref url_data) = self.url_data {
            if let (Some(scene), Some(script)) = (&url_data.scene_name, &url_data.script_entry) {
                match executor::execute_script(&self.executor_config, scene, script) {
                    Ok(result) => {
                        tracing::info!("脚本执行成功: {}", result.message);
                        self.status_message = Some("脚本启动成功".to_string());
                        self.last_execution = Some(result);
                    }
                    Err(e) => {
                        tracing::error!("脚本执行失败: {}", e);
                        self.status_message = Some(format!("执行失败: {}", e));
                        self.last_execution = None;
                    }
                }
            } else {
                self.status_message = Some("URL 缺少 scene 或 script 参数".to_string());
            }
        }
    }

    pub fn url_data(&self) -> Option<&UrlSchemeData> {
        self.url_data.as_ref()
    }

    pub fn status_message(&self) -> Option<&String> {
        self.status_message.as_ref()
    }

    pub fn is_registered(&self) -> bool {
        self.is_registered
    }

    pub fn executor_config(&self) -> &ExecutorConfig {
        &self.executor_config
    }

    pub fn executor_config_mut(&mut self) -> &mut ExecutorConfig {
        &mut self.executor_config
    }

    pub fn last_execution(&self) -> Option<&ExecutionResult> {
        self.last_execution.as_ref()
    }

    pub fn is_url_mode(&self) -> bool {
        self.is_url_mode
    }

    pub fn has_executed(&self) -> bool {
        self.has_executed
    }

    pub fn can_close_now(&self) -> bool {
        // 如果不是 URL 模式，永远不自动关闭
        if !self.is_url_mode {
            return false;
        }

        // 如果执行失败，不关闭
        if self.last_execution.is_none() && self.has_executed {
            return false;
        }

        // 如果执行成功，检查是否已经过了最小显示时间
        if let Some(exec_time) = self.execution_time {
            if exec_time.elapsed() >= self.min_display_duration {
                return true;
            }
        }

        false
    }
}
