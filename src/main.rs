#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use afsim_linker::{cli, run_gui};

fn main() {
    // 初始化日志系统
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::from_default_env()
                .add_directive(tracing::Level::INFO.into()),
        )
        .init();

    let command = cli::parse_args();
    let should_run = cli::handle_command(command.clone());

    if should_run {
        let url = cli::get_url_from_command(&command);

        // 启动 GUI
        if let Err(e) = run_gui(url) {
            tracing::error!("应用启动失败: {}", e);
            std::process::exit(1);
        }
    }
}
