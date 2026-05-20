use anyhow::Result;
use eframe::egui;
use std::path::PathBuf;

/// 加载中文字体：使用内嵌 SimHei（黑体），确保跨平台中文显示
fn load_cjk_font() -> Vec<u8> {
    include_bytes!("../assets/simhei.ttf").to_vec()
}

pub fn run(config_path: PathBuf, config: serde_json_lenient::Value) -> Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([900.0, 700.0]),
        ..Default::default()
    };

    eframe::run_native(
        "Zed 配置编辑器",
        options,
        Box::new(|cc| {
            let ctx = &cc.egui_ctx;
            let mut fonts = egui::FontDefinitions::default();

            let font_bytes = load_cjk_font();
            fonts.font_data.insert(
                "CJK".to_owned(),
                std::sync::Arc::new(egui::FontData::from_owned(font_bytes)),
            );

            fonts
                .families
                .get_mut(&egui::FontFamily::Proportional)
                .unwrap()
                .insert(0, "CJK".to_owned());

            fonts
                .families
                .get_mut(&egui::FontFamily::Monospace)
                .unwrap()
                .push("CJK".to_owned());

            ctx.set_fonts(fonts);

            Ok(Box::new(App::new(config_path, config)))
        }),
    )?; // 传播错误

    Ok(())
}

// ----- App 定义 -----
struct App {
    config_path: PathBuf,
    config: serde_json_lenient::Value,
    error_message: Option<String>,
}

impl App {
    fn new(config_path: PathBuf, config: serde_json_lenient::Value) -> Self {
        Self {
            config_path,
            config,
            error_message: None,
        }
    }

    fn save(&mut self) {
        match crate::core::save_config(&self.config_path, &self.config) {
            Ok(()) => self.error_message = Some("配置已保存".into()),
            Err(e) => self.error_message = Some(format!("保存失败: {}", e)),
        }
    }
}

impl eframe::App for App {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        egui::Panel::top("menu").show_inside(ui, |ui| {
            egui::MenuBar::new().ui(ui, |ui| {
                if ui.button("保存").clicked() {
                    self.save();
                }
                if let Some(msg) = &self.error_message {
                    ui.label(msg);
                }
            });
        });

        egui::CentralPanel::default().show_inside(ui, |ui| {
            egui::ScrollArea::vertical().show(ui, |ui| {
                show_json_value(ui, &mut self.config, 0);
            });
        });
    }
}

// ---------- 递归显示 JSON ----------
fn show_json_value(ui: &mut egui::Ui, value: &mut serde_json_lenient::Value, depth: usize) {
    match value {
        serde_json_lenient::Value::Null => {
            ui.label("null");
        }
        serde_json_lenient::Value::Bool(b) => {
            let mut checked = *b;
            if ui.checkbox(&mut checked, "").changed() {
                *b = checked;
            }
        }
        serde_json_lenient::Value::Number(n) => {
            let mut text = n.to_string();
            let response = ui.text_edit_singleline(&mut text);
            if response.changed() {
                // 使用 FromStr trait 解析数字
                if let Ok(new_n) = text.parse::<serde_json_lenient::Number>() {
                    *n = new_n;
                }
            }
        }
        serde_json_lenient::Value::String(s) => {
            ui.text_edit_singleline(s);
        }
        serde_json_lenient::Value::Array(arr) => {
            ui.collapsing("数组", |ui| {
                for (i, val) in arr.iter_mut().enumerate() {
                    ui.horizontal(|ui| {
                        ui.label(format!("[{}]", i));
                        show_json_value(ui, val, depth + 1);
                    });
                }
            });
        }
        serde_json_lenient::Value::Object(obj) => {
            // obj 是 &mut Map，可以直接遍历
            let keys: Vec<String> = obj.keys().cloned().collect(); // 先收集键，避免借用冲突
            for key in keys {
                if let Some(child) = obj.get_mut(&key) {
                    ui.collapsing(key, |ui| {
                        show_json_value(ui, child, depth + 1);
                    });
                }
            }
        }
    }
}
