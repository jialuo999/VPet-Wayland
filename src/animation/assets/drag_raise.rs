// ===== 依赖导入 =====
use std::path::{Path, PathBuf};

use crate::stats::PetMode;

use super::common::{
    collect_dir_paths, collect_png_files, dir_name_matches_mode, load_frames_with_fallback,
    Segment,
};

// ===== Drag Raise 资源选择 =====
pub(crate) fn collect_drag_raise_loop_files(
    raise_dynamic_root: &Path,
    mode: PetMode,
) -> Vec<PathBuf> {
    load_frames_with_fallback(raise_dynamic_root, mode, Segment::Single)
}

pub(crate) fn collect_drag_raise_start_files(raise_static_root: &Path, mode: PetMode) -> Vec<PathBuf> {
    let mut mode_dirs: Vec<PathBuf> = collect_dir_paths(raise_static_root)
        .iter()
        .filter(|path| {
            let dir_name = path.file_name().and_then(|s| s.to_str()).unwrap_or_default();
            dir_name.to_ascii_lowercase().starts_with("a_") && dir_name_matches_mode(dir_name, mode)
        })
        .cloned()
        .collect();

    if mode_dirs.is_empty() && mode != PetMode::Nomal {
        mode_dirs = collect_dir_paths(raise_static_root)
            .iter()
            .filter(|path| {
                let dir_name = path.file_name().and_then(|s| s.to_str()).unwrap_or_default();
                dir_name.to_ascii_lowercase().starts_with("a_")
                    && dir_name_matches_mode(dir_name, PetMode::Nomal)
            })
            .cloned()
            .collect();
    }

    if mode_dirs.is_empty() && mode != PetMode::Happy {
        mode_dirs = collect_dir_paths(raise_static_root)
            .iter()
            .filter(|path| {
                let dir_name = path.file_name().and_then(|s| s.to_str()).unwrap_or_default();
                dir_name.to_ascii_lowercase().starts_with("a_")
                    && dir_name_matches_mode(dir_name, PetMode::Happy)
            })
            .cloned()
            .collect();
    }

    mode_dirs
        .iter()
        .filter_map(|dir| {
            let files = collect_png_files(dir).ok()?;
            if files.is_empty() {
                None
            } else {
                Some(files)
            }
        })
        .next()
        .unwrap_or_default()
}

pub(crate) fn collect_drag_raise_static_b_variants(
    raise_static_root: &Path,
    mode: PetMode,
) -> Vec<Vec<PathBuf>> {
    let mut mode_dirs: Vec<PathBuf> = collect_dir_paths(raise_static_root)
        .iter()
        .filter(|path| {
            let dir_name = path.file_name().and_then(|s| s.to_str()).unwrap_or_default();
            dir_name.to_ascii_lowercase().starts_with("b_") && dir_name_matches_mode(dir_name, mode)
        })
        .cloned()
        .collect();

    if mode_dirs.is_empty() && mode != PetMode::Nomal {
        mode_dirs = collect_dir_paths(raise_static_root)
            .iter()
            .filter(|path| {
                let dir_name = path.file_name().and_then(|s| s.to_str()).unwrap_or_default();
                dir_name.to_ascii_lowercase().starts_with("b_")
                    && dir_name_matches_mode(dir_name, PetMode::Nomal)
            })
            .cloned()
            .collect();
    }

    if mode_dirs.is_empty() && mode != PetMode::Happy {
        mode_dirs = collect_dir_paths(raise_static_root)
            .iter()
            .filter(|path| {
                let dir_name = path.file_name().and_then(|s| s.to_str()).unwrap_or_default();
                dir_name.to_ascii_lowercase().starts_with("b_")
                    && dir_name_matches_mode(dir_name, PetMode::Happy)
            })
            .cloned()
            .collect();
    }

    mode_dirs
        .iter()
        .filter_map(|dir| {
            let files = collect_png_files(dir).ok()?;
            if files.is_empty() {
                None
            } else {
                Some(files)
            }
        })
        .collect()
}

pub(crate) fn collect_drag_raise_end_variants(
    raise_static_root: &Path,
    mode: PetMode,
) -> Vec<Vec<PathBuf>> {
    let mut mode_dirs: Vec<PathBuf> = collect_dir_paths(raise_static_root)
        .iter()
        .filter(|path| {
            let dir_name = path.file_name().and_then(|s| s.to_str()).unwrap_or_default();
            dir_name.to_ascii_lowercase().starts_with("c_") && dir_name_matches_mode(dir_name, mode)
        })
        .cloned()
        .collect();

    if mode_dirs.is_empty() && mode != PetMode::Happy {
        mode_dirs = collect_dir_paths(raise_static_root)
            .iter()
            .filter(|path| {
                let dir_name = path.file_name().and_then(|s| s.to_str()).unwrap_or_default();
                dir_name.to_ascii_lowercase().starts_with("c_")
                    && dir_name_matches_mode(dir_name, PetMode::Happy)
            })
            .cloned()
            .collect();
    }

    mode_dirs
        .iter()
        .filter_map(|dir| {
            let files = collect_png_files(dir).ok()?;
            if files.is_empty() {
                None
            } else {
                Some(files)
            }
        })
        .collect()
}
