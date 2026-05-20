use anyhow::{Context, Result};
use std::fs;
use std::path::PathBuf;

/// 加载 Zed 的 settings.json，返回解析后的 JSON 值
pub fn load_config() -> Result<(PathBuf, serde_json_lenient::Value)> {
    let config_path = dirs::config_dir()
        .context("无法获取配置目录")?
        .join("Zed")
        .join("settings.json");

    let content = fs::read_to_string(&config_path)
        .with_context(|| format!("无法读取配置文件: {}", config_path.display()))?;

    let value = serde_json_lenient::from_str(&content).context("解析 JSONC 失败")?;

    Ok((config_path, value))
}

/// 保存配置到原文件（自动备份）
pub fn save_config(config_path: &PathBuf, value: &serde_json_lenient::Value) -> Result<()> {
    let new_content = serde_json::to_string_pretty(value).context("序列化 JSON 失败")?;

    // 备份
    let backup_path = config_path.with_file_name(format!(
        "settings.json.bak_{}",
        std::time::UNIX_EPOCH.elapsed().unwrap().as_secs()
    ));
    fs::write(&backup_path, &new_content).context("备份配置文件失败")?;

    // 写入原文件
    fs::write(config_path, &new_content).context("写入配置文件失败")?;

    Ok(())
}
