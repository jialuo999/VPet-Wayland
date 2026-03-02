// ===== 依赖导入 =====
use anyhow::Context;
use notify::{Event, EventKind, RecommendedWatcher, RecursiveMode, Watcher};
use once_cell::sync::Lazy;
use std::path::Path;
use std::sync::Mutex;

use super::defaults::RUNTIME_CONFIG_FILE;
use super::loader::runtime_config_path;

// ===== 配置监听器存储 =====
static CONFIG_WATCHERS: Lazy<Mutex<Vec<RecommendedWatcher>>> = Lazy::new(|| Mutex::new(Vec::new()));

// ===== 配置热更新入口 =====
pub fn start_panel_config_watcher<F>(on_change: F) -> anyhow::Result<()>
where
	F: Fn() + Send + Sync + 'static,
{
	// 监听配置文件所在目录，筛选目标文件变更事件
	let config_path = runtime_config_path();
	let watch_target = config_path
		.parent()
		.unwrap_or_else(|| Path::new("."))
		.to_path_buf();
	let config_file_name = config_path
		.file_name()
		.map(|name| name.to_os_string())
		.unwrap_or_else(|| RUNTIME_CONFIG_FILE.into());

	let mut watcher = RecommendedWatcher::new(
		move |result: Result<Event, notify::Error>| {
			let event = match result {
				Ok(event) => event,
				Err(err) => {
					eprintln!("配置监听错误：{}", err);
					return;
				}
			};

			if !matches!(
				event.kind,
				EventKind::Create(_)
					| EventKind::Modify(_)
					| EventKind::Remove(_)
					| EventKind::Any
			) {
				return;
			}

			let is_target = event.paths.is_empty()
				|| event
					.paths
					.iter()
					.any(|path| path.file_name() == Some(config_file_name.as_os_str()));
			if is_target {
				on_change();
			}
		},
		notify::Config::default(),
	)
	.with_context(|| "创建配置文件监听器失败")?;

	watcher
		.watch(&watch_target, RecursiveMode::NonRecursive)
		.with_context(|| format!("监听配置目录失败：{}", watch_target.display()))?;

	let mut watchers = CONFIG_WATCHERS
		.lock()
		.map_err(|_| anyhow::anyhow!("配置监听器存储锁被污染"))?;
	watchers.push(watcher);

	Ok(())
}
