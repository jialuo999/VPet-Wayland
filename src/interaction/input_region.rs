// ===== 依赖导入 =====
use gtk4::cairo::Region;
use gtk4::prelude::*;
use gtk4::{ApplicationWindow, Image};

use crate::config::INPUT_DEBUG_LOG;

// ===== 输入区域：仅透明像素不响应点击 =====
pub fn setup_image_input_region(
    window: &ApplicationWindow,
    image: &Image,
    pixbuf: &gdk_pixbuf::Pixbuf,
) {
    let Some(surface) = window.surface() else {
        if INPUT_DEBUG_LOG {
            eprintln!("[input-region] skipped: window surface is None");
        }
        return;
    };

    let alloc = image.allocation();
    let (offset_x, offset_y, render_w, render_h) =
        (alloc.x(), alloc.y(), alloc.width(), alloc.height());

    let region = if render_w > 0 && render_h > 0 {
        create_region_from_pixbuf_scaled(pixbuf, offset_x, offset_y, render_w, render_h)
    } else {
        let full = gtk4::cairo::RectangleInt::new(0, 0, pixbuf.width(), pixbuf.height());
        Region::create_rectangle(&full)
    };

    surface.set_input_region(&region);
    if INPUT_DEBUG_LOG {
        eprintln!(
            "[input-region] applied: pixbuf={}x{}, render=({},{} {}x{}), has_alpha={}, region_empty={}",
            pixbuf.width(),
            pixbuf.height(),
            offset_x,
            offset_y,
            render_w,
            render_h,
            pixbuf.has_alpha(),
            region.is_empty()
        );
    }
}

// ===== 透明通道扫描：生成真实可点击 Region =====
fn create_region_from_pixbuf_scaled(
    pixbuf: &gdk_pixbuf::Pixbuf,
    offset_x: i32,
    offset_y: i32,
    render_w: i32,
    render_h: i32,
) -> Region {
    let src_w = pixbuf.width();
    let src_h = pixbuf.height();

    if !pixbuf.has_alpha() {
        let full = gtk4::cairo::RectangleInt::new(offset_x, offset_y, render_w, render_h);
        return Region::create_rectangle(&full);
    }

    let Some(pixel_bytes) = pixbuf.pixel_bytes() else {
        let full = gtk4::cairo::RectangleInt::new(offset_x, offset_y, render_w, render_h);
        return Region::create_rectangle(&full);
    };

    let bytes = pixel_bytes.as_ref();
    let channels = pixbuf.n_channels() as usize;
    let rowstride = pixbuf.rowstride() as usize;
    let alpha_idx = channels - 1;
    let region = Region::create();

    for dy in 0..render_h {
        let sy = ((dy as i64 * src_h as i64) / render_h as i64) as usize;
        let mut run_start: Option<i32> = None;

        for dx in 0..render_w {
            let sx = ((dx as i64 * src_w as i64) / render_w as i64) as usize;
            let offset = sy * rowstride + sx * channels;
            let alpha = if offset + alpha_idx < bytes.len() {
                bytes[offset + alpha_idx]
            } else {
                0
            };
            match (run_start, alpha > 0) {
                (None, true) => run_start = Some(dx),
                (Some(start), false) => {
                    let rect =
                        gtk4::cairo::RectangleInt::new(offset_x + start, offset_y + dy, dx - start, 1);
                    let _ = region.union_rectangle(&rect);
                    run_start = None;
                }
                _ => {}
            }
        }

        if let Some(start) = run_start {
            let rect =
                gtk4::cairo::RectangleInt::new(offset_x + start, offset_y + dy, render_w - start, 1);
            let _ = region.union_rectangle(&rect);
        }
    }

    if region.is_empty() {
        let full = gtk4::cairo::RectangleInt::new(offset_x, offset_y, render_w, render_h);
        Region::create_rectangle(&full)
    } else {
        region
    }
}
