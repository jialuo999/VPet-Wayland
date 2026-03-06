// ===== config 子模块声明 =====
mod defaults;
mod loader;
mod model;
mod watcher;

// ===== 常量导出 =====
pub use defaults::{
    APP_ID, CAROUSEL_INTERVAL_MS, DEFAULT_PIXEL_SIZE, DRAG_ALLOW_OFFSCREEN, DRAG_LONG_PRESS_MS,
    INPUT_DEBUG_LOG,
};

// ===== 类型与函数导出 =====
pub use loader::{load_animation_path_config, load_panel_debug_config};
pub use model::{AnimationPathConfig, PanelDebugConfig};
pub use watcher::start_panel_config_watcher;
