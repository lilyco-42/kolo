use anyhow::{Context, Result};
use serde_json;
use std::fs;
use std::path::PathBuf;
fn main() -> Result<()> {
    let user: PathBuf = dirs::config_dir().context("无法获取用户主目录")?;
    dbg!(&user);

    // 拼接路径，更通用的做法是用 dirs::config_dir()
    let config_path = user.join("Zed").join("settings.json");
    // 读取文件
    let content = fs::read_to_string(&config_path)
        .with_context(|| format!("无法读取配置文件: {}", config_path.display()))?;
    // 解析 JSON
    let config: serde_json_lenient::Value =
        serde_json_lenient::from_str(&content).context("解析 JSON 失败")?;
    let new_file = serde_json::to_string_pretty(&config).context("序列化 JSON 失败")?;
    // 写入前先备份源文件
    // 备份文件路径为 config_path.with_extension("bak")
    let backup_path = config_path.with_extension(format!(
        "json.bak_{}",
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs()
    ));
    fs::write(&backup_path, content).context("备份配置文件失败")?;
    fs::write(&config_path, new_file).context("写入配置文件失败")?;

    dbg!(config);
    Ok(())
}
