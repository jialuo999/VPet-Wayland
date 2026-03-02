// ===== 依赖导入 =====
use std::path::{Path, PathBuf};

use crate::stats::PetMode;

use super::common::{
    collect_dir_paths, collect_png_files, dir_name_matches_mode, load_frames_with_fallback,
    pseudo_random_index, Segment,
};

// ===== Pinch 资源选择 =====
fn collect_pinch_stage_variants(
    pinch_root: &Path,
    mode: PetMode,
    stage_prefix: &str,
) -> Vec<Vec<PathBuf>> {
    let mode_dirs: Vec<PathBuf> = collect_dir_paths(pinch_root)
        .iter()
        .filter(|path| {
            path.file_name()
                .and_then(|s| s.to_str())
                .map(|name| dir_name_matches_mode(name, mode))
                .unwrap_or(false)
        })
        .cloned()
        .collect();

    let selected_mode_dir = if mode_dirs.is_empty() && mode != PetMode::Nomal {
        collect_dir_paths(pinch_root)
            .into_iter()
            .find(|path| {
                path.file_name()
                    .and_then(|s| s.to_str())
                    .map(|name| dir_name_matches_mode(name, PetMode::Nomal))
                    .unwrap_or(false)
            })
    } else {
        mode_dirs.into_iter().next()
    }
    .or_else(|| {
        if mode != PetMode::Happy {
            collect_dir_paths(pinch_root)
                .into_iter()
                .find(|path| {
                    path.file_name()
                        .and_then(|s| s.to_str())
                        .map(|name| dir_name_matches_mode(name, PetMode::Happy))
                        .unwrap_or(false)
                })
        } else {
            None
        }
    });

    let Some(mode_dir) = selected_mode_dir else {
        return Vec::new();
    };

    collect_dir_paths(&mode_dir)
        .into_iter()
        .filter(|path| {
            path.file_name()
                .and_then(|s| s.to_str())
                .map(|name| name.to_ascii_lowercase().starts_with(&stage_prefix.to_ascii_lowercase()))
                .unwrap_or(false)
        })
        .filter_map(|path| {
            let files = collect_png_files(&path).ok()?;
            if files.is_empty() {
                None
            } else {
                Some(files)
            }
        })
        .collect()
}

pub(crate) fn collect_pinch_start_files(pinch_root: &Path, mode: PetMode) -> Vec<PathBuf> {
    let files = load_frames_with_fallback(pinch_root, mode, Segment::A);
    if files.is_empty() {
        let mut variants = collect_pinch_stage_variants(pinch_root, mode, "A");
        if variants.is_empty() {
            Vec::new()
        } else {
            variants.swap_remove(pseudo_random_index(variants.len()))
        }
    } else {
        files
    }
}

pub(crate) fn collect_pinch_loop_variants(pinch_root: &Path, mode: PetMode) -> Vec<Vec<PathBuf>> {
    collect_pinch_stage_variants(pinch_root, mode, "B")
}

pub(crate) fn collect_pinch_end_files(pinch_root: &Path, mode: PetMode) -> Vec<PathBuf> {
    let files = load_frames_with_fallback(pinch_root, mode, Segment::C);
    if files.is_empty() {
        let mut variants = collect_pinch_stage_variants(pinch_root, mode, "C");
        if variants.is_empty() {
            Vec::new()
        } else {
            variants.swap_remove(pseudo_random_index(variants.len()))
        }
    } else {
        files
    }
}
