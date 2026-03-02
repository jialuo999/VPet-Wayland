// ===== 依赖导入 =====
use gtk4::prelude::*;
use gtk4::{ApplicationWindow, GestureClick, Image};

use crate::config::INPUT_DEBUG_LOG;

// ===== 输入诊断探针（调试时输出点击日志） =====
pub fn setup_input_probe(window: &ApplicationWindow, image: &Image) {
    if !INPUT_DEBUG_LOG {
        return;
    }

    let win_click = GestureClick::new();
    win_click.connect_pressed(|_, _, x, y| {
        eprintln!("[probe] window click at ({x:.1}, {y:.1})");
    });
    window.add_controller(win_click);

    let img_click = GestureClick::new();
    img_click.connect_pressed(|_, _, x, y| {
        eprintln!("[probe] image click at ({x:.1}, {y:.1})");
    });
    image.add_controller(img_click);
}
