use crate::app::ClientLinkerApp;
use crate::config::APP_VERSION;

pub fn render_welcome_view(app: &mut ClientLinkerApp, ctx: &egui::Context) {
    // 如果是 URL 模式，显示加载界面
    if app.is_url_mode() {
        tracing::debug!("渲染加载界面（URL 模式）");
        render_loading_view(app, ctx);
        return;
    }

    tracing::debug!("渲染设置界面（普通模式）");

    // 底部状态栏
    egui::TopBottomPanel::bottom("status_panel")
        .resizable(false)
        .exact_height(30.0)
        .show(ctx, |ui| {
            ui.horizontal_centered(|ui| {
                ui.with_layout(egui::Layout::left_to_right(egui::Align::Center), |ui| {
                    // 状态信息
                    render_registration_status_inline(ui, app.is_registered());
                });

                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    ui.colored_label(egui::Color32::GRAY, format!("v{}", APP_VERSION));
                });
            });
        });

    // 主内容区域 - 使用滚动区域
    egui::CentralPanel::default().show(ctx, |ui| {
        egui::ScrollArea::vertical().show(ui, |ui| {
            ui.vertical(|ui| {
                ui.add_space(10.0);

                // 推演程序配置区域
                render_executor_config_section(ui, app);

                ui.add_space(15.0);

                // URL Scheme 配置区域
                render_url_scheme_section(ui, app);

                ui.add_space(15.0);

                // 配置说明放在所有配置的最后
                ui.label(
                    egui::RichText::new(
                        "提示：URL 调用格式为 afsim://run?scene=场景名称&script=脚本入口",
                    )
                    .color(egui::Color32::GRAY)
                    .size(11.0),
                );

                ui.add_space(10.0);
            });
        });
    });
}

fn render_loading_view(app: &ClientLinkerApp, ctx: &egui::Context) {
    egui::CentralPanel::default().show(ctx, |ui| {
        ui.vertical_centered(|ui| {
            ui.add_space(100.0);

            // 标题
            ui.heading("AFSim 链接器");
            ui.add_space(30.0);

            // 加载动画（使用 spinner）
            ui.add(egui::Spinner::new().size(40.0));
            ui.add_space(20.0);

            // 加载文本
            ui.label(
                egui::RichText::new("正在加载推演场景...")
                    .size(16.0)
                    .color(egui::Color32::from_rgb(100, 100, 100)),
            );

            ui.add_space(20.0);

            // 显示场景信息
            if let Some(url_data) = app.url_data() {
                if let Some(scene) = &url_data.scene_name {
                    ui.label(
                        egui::RichText::new(format!("场景: {}", scene))
                            .size(14.0)
                            .color(egui::Color32::from_rgb(80, 80, 80)),
                    );
                }
                if let Some(script) = &url_data.script_entry {
                    ui.label(
                        egui::RichText::new(format!("脚本: {}", script))
                            .size(14.0)
                            .color(egui::Color32::from_rgb(80, 80, 80)),
                    );
                }
            }

            ui.add_space(30.0);

            // 显示执行结果或错误信息
            if let Some(status) = app.status_message() {
                let is_error = status.contains("失败") || status.contains("错误");
                let color = if is_error {
                    egui::Color32::RED
                } else {
                    egui::Color32::GREEN
                };
                ui.colored_label(color, status);
            }
        });
    });
}

fn render_executor_config_section(ui: &mut egui::Ui, app: &mut ClientLinkerApp) {
    ui.heading("推演配置");
    ui.separator();
    ui.add_space(5.0);

    // 可执行程序路径
    ui.label("可执行程序路径：");
    ui.horizontal(|ui| {
        let config = app.executor_config_mut();
        let text_edit = egui::TextEdit::singleline(&mut config.executable_path)
            .hint_text("例如：C:\\AFSim\\afsim.exe");
        ui.add_sized([ui.available_width() - 80.0, 20.0], text_edit);

        if ui.button("📁 浏览").clicked() {
            if let Some(path) = rfd::FileDialog::new().pick_file() {
                config.executable_path = path.display().to_string();
            }
        }
    });

    ui.add_space(10.0);

    // 脚本基础路径
    ui.label("脚本基础路径：");
    ui.horizontal(|ui| {
        let config = app.executor_config_mut();
        let text_edit = egui::TextEdit::singleline(&mut config.script_base_path)
            .hint_text("例如：D:\\projects\\scenarios");
        ui.add_sized([ui.available_width() - 80.0, 20.0], text_edit);

        if ui.button("📁 浏览").clicked() {
            if let Some(path) = rfd::FileDialog::new().pick_folder() {
                config.script_base_path = path.display().to_string();
            }
        }
    });

    ui.add_space(10.0);

    // 保存按钮和状态
    ui.horizontal(|ui| {
        if ui.button("💾 保存配置").clicked() {
            app.save_config_with_validation();
        }

        // 显示保存结果消息
        if let Some(status) = app.status_message() {
            if status.contains("配置已保存") {
                ui.colored_label(egui::Color32::GREEN, "配置已保存");
            } else if status.contains("保存失败") || status.contains("路径") {
                ui.colored_label(egui::Color32::RED, status);
            }
        }
    });
}

fn render_url_scheme_section(ui: &mut egui::Ui, app: &mut ClientLinkerApp) {
    ui.heading("协议配置");
    ui.separator();
    ui.add_space(5.0);

    // 操作按钮
    ui.horizontal(|ui| {
        if app.is_registered() {
            ui.add_enabled(false, egui::Button::new("注册协议"));

            if ui.button("卸载协议").clicked() {
                app.handle_unregister();
            }
        } else {
            if ui.button("注册协议").clicked() {
                app.handle_register();
            }

            ui.add_enabled(false, egui::Button::new("卸载协议"));
        }
    });
}

fn render_registration_status_inline(ui: &mut egui::Ui, is_registered: bool) {
    let (status_text, status_color) = if is_registered {
        ("协议已注册", egui::Color32::GREEN)
    } else {
        ("协议未注册", egui::Color32::GRAY)
    };

    ui.colored_label(status_color, status_text);
}
