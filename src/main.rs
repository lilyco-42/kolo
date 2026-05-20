mod core;
mod gui;

fn main() {
    // 先加载配置
    let (config_path, config) = match core::load_config() {
        Ok((path, val)) => (path, val),
        Err(e) => {
            eprintln!("启动失败: {}", e);
            return;
        }
    };

    // 启动 GUI
    if let Err(e) = gui::run(config_path, config) {
        eprintln!("GUI 错误: {}", e);
    }
}
