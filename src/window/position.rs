// ===== 依赖导入 =====
use gtk4::prelude::*;
use gtk4::ApplicationWindow;
use gtk4_layer_shell::{Edge, LayerShell};

use crate::settings::WindowPosition;

// ===== 窗口坐标工具 =====

// 读取窗口当前左上角坐标（统一转换为 Left/Top 语义，便于持久化）
pub fn current_window_left_top(window: &ApplicationWindow) -> (i32, i32) {
    let alloc = window.allocation();
    let win_w = alloc.width().max(1);
    let win_h = alloc.height().max(1);

    let (mon_w, mon_h) = window
        .surface()
        .and_then(|surface| {
            let display = surface.display();
            display.monitor_at_surface(&surface).map(|monitor| {
                let geometry = monitor.geometry();
                (geometry.width(), geometry.height())
            })
        })
        .unwrap_or((1920, 1080));

    let left = if window.is_anchor(Edge::Left) {
        window.margin(Edge::Left)
    } else if window.is_anchor(Edge::Right) {
        mon_w - win_w - window.margin(Edge::Right)
    } else {
        window.margin(Edge::Left)
    };

    let top = if window.is_anchor(Edge::Top) {
        window.margin(Edge::Top)
    } else if window.is_anchor(Edge::Bottom) {
        mon_h - win_h - window.margin(Edge::Bottom)
    } else {
        window.margin(Edge::Top)
    };

    (left, top)
}

// 应用已保存的位置：切换为 Left+Top 锚定并设置 margin
pub fn apply_window_position(window: &ApplicationWindow, position: WindowPosition) {
    window.set_anchor(Edge::Left, true);
    window.set_anchor(Edge::Top, true);
    window.set_anchor(Edge::Right, false);
    window.set_anchor(Edge::Bottom, false);
    window.set_margin(Edge::Left, position.left);
    window.set_margin(Edge::Top, position.top);
}
