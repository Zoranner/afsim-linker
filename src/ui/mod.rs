pub mod components;
pub mod fonts;

use crate::app::ClientLinkerApp;
use crate::config::AppConfig;
use anyhow::Result;

pub fn run_gui(url: Option<String>) -> Result<()> {
    let config = AppConfig::new();

    let viewport_builder = egui::ViewportBuilder::default()
        .with_inner_size([400.0, 600.0])
        .with_min_inner_size([400.0, 600.0])
        .with_max_inner_size([400.0, 600.0])
        .with_resizable(false)
        .with_title(&config.window_title);

    let options = eframe::NativeOptions {
        viewport: viewport_builder,
        hardware_acceleration: eframe::HardwareAcceleration::Preferred,
        ..Default::default()
    };

    let app = ClientLinkerApp::new(url);

    eframe::run_native(
        &config.window_title,
        options,
        Box::new(move |cc| {
            // 配置中文字体支持
            fonts::setup_fonts(&cc.egui_ctx);
            Ok(Box::new(app))
        }),
    )
    .map_err(|e| anyhow::anyhow!("GUI 启动失败: {}", e))?;

    Ok(())
}

impl eframe::App for ClientLinkerApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // 如果是 URL 模式且还没执行过脚本，先执行脚本
        if self.is_url_mode() && !self.has_executed() {
            tracing::info!("URL 模式：开始执行脚本");
            self.execute_from_url();
        }

        // 如果可以关闭了（执行成功且已经过了最小显示时间），则退出
        if self.can_close_now() {
            tracing::info!("URL 模式：准备关闭窗口");
            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
            return;
        }

        // 渲染 UI
        components::render_welcome_view(self, ctx);

        // 如果在 URL 模式下，持续请求重绘以检查是否可以关闭
        if self.is_url_mode() {
            ctx.request_repaint();
        }
    }
}
