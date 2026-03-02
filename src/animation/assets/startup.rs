// ===== 依赖导入 =====
use std::fs;
use std::path::{Path, PathBuf};

use crate::stats::PetMode;

use super::common::{collect_png_files, dir_name_matches_mode, pseudo_random_index};

// ===== Startup 资源选择 =====
pub(crate) fn choose_startup_animation_files(
    startup_root: &Path,
    mode: PetMode,
) -> Option<Vec<PathBuf>> {
    let startup_dirs: Vec<PathBuf> = fs::read_dir(startup_root)
        .ok()?
        .filter_map(|entry| {
            let entry = entry.ok()?;
            let path = entry.path();
            if !path.is_dir() {
                return None;
            }

            let dir_name = path.file_name()?.to_str()?;
            let include_dir = if mode == PetMode::Happy {
                dir_name_matches_mode(dir_name, PetMode::Happy)
                    || dir_name.eq_ignore_ascii_case("26new")
            } else {
                dir_name_matches_mode(dir_name, mode)
            };
            if !include_dir {
                return None;
            }

            Some(path)
        })
        .collect();

    if startup_dirs.is_empty() {
        return None;
    }

    let mut available_variants: Vec<Vec<PathBuf>> = startup_dirs
        .iter()
        .filter_map(|dir| {
            let files = collect_png_files(dir).ok()?;
            if files.is_empty() {
                None
            } else {
                Some(files)
            }
        })
        .collect();

    if available_variants.is_empty() {
        return None;
    }

    let selected_index = pseudo_random_index(available_variants.len());
    Some(available_variants.swap_remove(selected_index))
}
