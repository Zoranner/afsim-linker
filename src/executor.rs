use crate::config::ExecutorConfig;
use anyhow::{Context, Result};
use std::path::Path;
use std::process::{Command, Stdio};

#[cfg(windows)]
use std::os::windows::process::CommandExt;

/// 执行脚本
pub fn execute_script(
    config: &ExecutorConfig,
    scene_name: &str,
    script_entry: &str,
) -> Result<ExecutionResult> {
    // 验证配置
    if !config.is_valid() {
        return Err(anyhow::anyhow!(
            "配置无效：请检查可执行程序路径和脚本基础路径"
        ));
    }

    // 构建脚本路径
    let script_path = config.build_script_path(scene_name, script_entry);

    // 验证脚本是否存在
    if !script_path.exists() {
        return Err(anyhow::anyhow!("脚本文件不存在: {}", script_path.display()));
    }

    tracing::info!(
        "执行脚本: {} {}",
        config.executable_path,
        script_path.display()
    );

    // 执行命令
    let mut command = Command::new(&config.executable_path);
    command
        .arg(&script_path)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());

    // Windows: 隐藏控制台窗口
    #[cfg(windows)]
    {
        const CREATE_NO_WINDOW: u32 = 0x08000000;
        command.creation_flags(CREATE_NO_WINDOW);
    }

    let output = command.spawn().context("启动进程失败")?;

    // 获取进程 ID
    let pid = output.id();

    tracing::info!("脚本已启动，PID: {}", pid);

    Ok(ExecutionResult {
        message: format!(
            "成功启动脚本\n场景: {}\n脚本: {}\n进程 ID: {}",
            scene_name, script_entry, pid
        ),
        pid: Some(pid),
        script_path: script_path.to_string_lossy().to_string(),
    })
}

/// 执行结果
#[derive(Debug, Clone)]
pub struct ExecutionResult {
    pub message: String,
    pub pid: Option<u32>,
    pub script_path: String,
}

/// 验证路径是否存在
pub fn validate_path(path: &str) -> bool {
    !path.is_empty() && Path::new(path).exists()
}
