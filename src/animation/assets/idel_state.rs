use std::path::{Path, PathBuf};

use crate::stats::PetMode;

use super::common::{
    collect_dir_paths, collect_png_files, collect_png_files_recursive_filtered,
    collect_png_variant_dirs_recursive, dir_name_matches_mode, pseudo_random_index,
};

#[derive(Clone, Copy)]
pub(crate) enum IdelStateSegment {
    A,
    B,
    C,
    Single,
}

fn mode_candidates(mode: PetMode) -> [PetMode; 3] {
    [mode, PetMode::Nomal, PetMode::Happy]
}

fn mode_keyword_in_path(path: &Path, mode: PetMode) -> bool {
    path.components().any(|component| {
        component
            .as_os_str()
            .to_str()
            .map(|name| dir_name_matches_mode(name, mode))
            .unwrap_or(false)
    })
}

fn path_has_any_mode_keyword(path: &Path) -> bool {
    path.components().any(|component| {
        component
            .as_os_str()
            .to_str()
            .map(|name| {
                let lower = name.to_ascii_lowercase();
                lower.contains("happy")
                    || lower.contains("nomal")
                    || lower.contains("poorcondition")
                    || lower.contains("ill")
            })
            .unwrap_or(false)
    })
}

fn path_matches_mode_or_agnostic(path: &Path, mode: PetMode) -> bool {
    mode_keyword_in_path(path, mode) || !path_has_any_mode_keyword(path)
}

fn segment_prefix(segment: IdelStateSegment) -> Option<&'static str> {
    match segment {
        IdelStateSegment::A => Some("a"),
        IdelStateSegment::B => Some("b"),
        IdelStateSegment::C => Some("c"),
        IdelStateSegment::Single => None,
    }
}

fn component_matches_segment(name: &str, prefix: &str) -> bool {
    let normalized = name.to_ascii_lowercase();
    if normalized == prefix {
        return true;
    }

    normalized
        .strip_prefix(prefix)
        .and_then(|rest| rest.chars().next())
        .map(|next| !next.is_ascii_alphanumeric())
        .unwrap_or(false)
}

fn path_matches_segment(path: &Path, root: &Path, segment: IdelStateSegment) -> bool {
    let Ok(relative) = path.strip_prefix(root) else {
        return false;
    };

    match segment_prefix(segment) {
        Some(prefix) => relative.components().any(|component| {
            component
                .as_os_str()
                .to_str()
                .map(|name| component_matches_segment(name, prefix))
                .unwrap_or(false)
        }),
        None => !relative.components().any(|component| {
            component
                .as_os_str()
                .to_str()
                .map(|name| {
                    component_matches_segment(name, "a")
                        || component_matches_segment(name, "b")
                        || component_matches_segment(name, "c")
                })
                .unwrap_or(false)
        }),
    }
}

fn collect_segment_variants_for_mode(
    root: &Path,
    mode: PetMode,
    segment: IdelStateSegment,
) -> Vec<Vec<PathBuf>> {
    let mut dirs: Vec<PathBuf> = collect_png_variant_dirs_recursive(root)
        .into_iter()
        .filter(|path| path_matches_mode_or_agnostic(path, mode))
        .filter(|path| path_matches_segment(path, root, segment))
        .collect();

    dirs.sort();
    dirs.into_iter()
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

fn collect_segment_variants(root: &Path, mode: PetMode, segment: IdelStateSegment) -> Vec<Vec<PathBuf>> {
    for candidate in mode_candidates(mode) {
        let variants = collect_segment_variants_for_mode(root, candidate, segment);
        if !variants.is_empty() {
            return variants;
        }
    }

    if matches!(segment, IdelStateSegment::Single) {
        let flat = collect_png_files_recursive_filtered(root, &[]).unwrap_or_default();
        if !flat.is_empty() {
            return vec![flat];
        }
    }

    Vec::new()
}

fn choose_variant(mut variants: Vec<Vec<PathBuf>>) -> Vec<PathBuf> {
    if variants.is_empty() {
        return Vec::new();
    }

    variants.swap_remove(pseudo_random_index(variants.len()))
}

pub(crate) fn collect_idel_action_names(idel_root: &Path) -> Vec<String> {
    let mut names: Vec<String> = collect_dir_paths(idel_root)
        .into_iter()
        .filter_map(|path| path.file_name().and_then(|name| name.to_str()).map(str::to_string))
        .collect();
    names.sort();
    names
}

pub(crate) fn load_idel_segment(
    idel_root: &Path,
    action_name: &str,
    mode: PetMode,
    segment: IdelStateSegment,
) -> Vec<PathBuf> {
    let action_root = idel_root.join(action_name);
    choose_variant(collect_segment_variants(&action_root, mode, segment))
}

pub(crate) fn load_idel_loop_variants(
    idel_root: &Path,
    action_name: &str,
    mode: PetMode,
) -> Vec<Vec<PathBuf>> {
    let action_root = idel_root.join(action_name);
    collect_segment_variants(&action_root, mode, IdelStateSegment::B)
}

pub(crate) fn load_state_segment(
    state_root: &Path,
    state_name: &str,
    mode: PetMode,
    segment: IdelStateSegment,
) -> Vec<PathBuf> {
    let root = state_root.join(state_name);
    choose_variant(collect_segment_variants(&root, mode, segment))
}

pub(crate) fn load_state_loop_variants(
    state_root: &Path,
    state_name: &str,
    mode: PetMode,
) -> Vec<Vec<PathBuf>> {
    let root = state_root.join(state_name);
    collect_segment_variants(&root, mode, IdelStateSegment::B)
}

pub(crate) fn load_switch_single(switch_root: &Path, mode: PetMode) -> Vec<PathBuf> {
    choose_variant(collect_segment_variants(switch_root, mode, IdelStateSegment::Single))
}
