//! 字体设置模块 - 处理跨平台中文字体支持

use egui::{FontDefinitions, FontFamily};

/// 设置跨平台的中文字体支持
pub fn setup_fonts(ctx: &egui::Context) {
    let mut fonts = FontDefinitions::default();

    // 使用系统字体支持中文
    // Windows 系统字体
    #[cfg(target_os = "windows")]
    {
        // 尝试加载 Windows 系统中文字体
        if let Ok(font_data) = std::fs::read("C:/Windows/Fonts/msyh.ttc") {
            fonts.font_data.insert(
                "microsoft_yahei".to_owned(),
                egui::FontData::from_owned(font_data),
            );

            // 将微软雅黑设为默认字体
            fonts
                .families
                .get_mut(&FontFamily::Proportional)
                .unwrap()
                .insert(0, "microsoft_yahei".to_owned());
        } else if let Ok(font_data) = std::fs::read("C:/Windows/Fonts/simsun.ttc") {
            fonts
                .font_data
                .insert("simsun".to_owned(), egui::FontData::from_owned(font_data));

            fonts
                .families
                .get_mut(&FontFamily::Proportional)
                .unwrap()
                .insert(0, "simsun".to_owned());
        }
    }

    // Linux 系统字体
    #[cfg(target_os = "linux")]
    {
        // 尝试加载 Linux 系统中文字体
        let font_paths = [
            // Noto CJK 字体（推荐）
            "/usr/share/fonts/opentype/noto/NotoSansCJK-Regular.ttc",
            "/usr/share/fonts/noto-cjk/NotoSansCJK-Regular.ttc",
            "/usr/share/fonts/truetype/noto/NotoSansCJK-Regular.ttc",
            // 文泉驿字体
            "/usr/share/fonts/truetype/wqy/wqy-microhei.ttc",
            "/usr/share/fonts/wqy-microhei/wqy-microhei.ttc",
            "/usr/share/fonts/truetype/wqy/wqy-zenhei.ttc",
            // Droid 字体
            "/usr/share/fonts/truetype/droid/DroidSansFallbackFull.ttf",
            // AR PL 字体
            "/usr/share/fonts/truetype/arphic/uming.ttc",
            "/usr/share/fonts/truetype/arphic/ukai.ttc",
        ];

        let mut font_loaded = false;
        for path in &font_paths {
            if let Ok(font_data) = std::fs::read(path) {
                let font_name = "chinese_font".to_owned();
                fonts
                    .font_data
                    .insert(font_name.clone(), egui::FontData::from_owned(font_data));

                fonts
                    .families
                    .get_mut(&FontFamily::Proportional)
                    .unwrap()
                    .insert(0, font_name.clone());

                fonts
                    .families
                    .get_mut(&FontFamily::Monospace)
                    .unwrap()
                    .insert(0, font_name);

                tracing::info!("Loaded Chinese font from: {}", path);
                font_loaded = true;
                break;
            }
        }

        if !font_loaded {
            tracing::warn!("No Chinese font found on Linux system. Chinese characters may not display correctly.");
            tracing::warn!("Please install fonts-noto-cjk or fonts-wqy-microhei package.");
        }
    }

    ctx.set_fonts(fonts);
    tracing::info!("System fonts configured for Chinese display");
}
