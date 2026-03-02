// ===== 依赖导入 =====
use std::fs;
use std::path::PathBuf;

use super::defaults::RUNTIME_CONFIG_FILE;
use super::model::{AnimationPathConfig, FileConfig, PanelDebugConfig};

// ===== 配置加载入口 =====
pub(crate) fn runtime_config_path() -> PathBuf {
	PathBuf::from(RUNTIME_CONFIG_FILE)
}

fn load_file_config() -> Option<FileConfig> {
	let path = runtime_config_path();
	let content = match fs::read_to_string(&path) {
		Ok(content) => content,
		Err(err) => {
			if err.kind() != std::io::ErrorKind::NotFound {
				eprintln!("读取配置文件失败（{}）：{}", path.display(), err);
			}
			return None;
		}
	};

	match toml::from_str::<FileConfig>(&content) {
		Ok(file_config) => Some(file_config),
		Err(err) => {
			eprintln!("解析配置文件失败（{}）：{}", path.display(), err);
			None
		}
	}
}

pub fn load_panel_debug_config() -> PanelDebugConfig {
	let default_config = PanelDebugConfig::default();
	load_file_config()
		.and_then(|file_config| file_config.panel)
		.unwrap_or_default()
		.merge_into(default_config)
		.sanitized()
}

pub fn load_animation_path_config() -> AnimationPathConfig {
	let default_config = AnimationPathConfig::default();
	load_file_config()
		.and_then(|file_config| file_config.animation)
		.unwrap_or_default()
		.merge_into(default_config)
		.sanitized()
}
