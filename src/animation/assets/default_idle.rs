// ===== 依赖导入 =====
use std::path::PathBuf;

use crate::config::AnimationPathConfig;
use crate::stats::PetMode;

use super::common::{body_asset_path, collect_png_files, collect_png_variant_dirs_recursive};

// ===== Default Idle 资源选择 =====
pub(crate) fn collect_default_happy_idle_variants(
    animation_config: &AnimationPathConfig,
) -> Result<Vec<Vec<PathBuf>>, String> {
    let mut variants = Vec::new();
    for variant in &animation_config.default_happy_idle_variants {
        let dir = body_asset_path(&animation_config.assets_body_root, variant);
        if !dir.is_dir() {
            return Err(format!("目录不存在：{}", dir.display()));
        }

        let mut variant_dirs = collect_png_variant_dirs_recursive(&dir);
        variant_dirs.sort();

        for variant_dir in variant_dirs {
            let files = collect_png_files(&variant_dir)?;
            if !files.is_empty() {
                variants.push(files);
            }
        }
    }
    Ok(variants)
}

pub(crate) fn collect_default_mode_idle_variants(
    animation_config: &AnimationPathConfig,
    mode: PetMode,
) -> Vec<Vec<PathBuf>> {
    if mode == PetMode::Happy {
        return collect_default_happy_idle_variants(animation_config).unwrap_or_default();
    }

    let root = match mode {
        PetMode::Nomal => body_asset_path(
            &animation_config.assets_body_root,
            &animation_config.default_nomal_idle_root,
        ),
        PetMode::PoorCondition => body_asset_path(
            &animation_config.assets_body_root,
            &animation_config.default_poor_condition_idle_root,
        ),
        PetMode::Ill => body_asset_path(
            &animation_config.assets_body_root,
            &animation_config.default_ill_idle_root,
        ),
        PetMode::Happy => unreachable!(),
    };

    collect_png_variant_dirs_recursive(&root)
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

fn flatten_variants_in_order(variants: &[Vec<PathBuf>]) -> Vec<PathBuf> {
    let mut files = Vec::new();
    for variant in variants {
        files.extend(variant.iter().cloned());
    }
    files
}

pub(crate) fn select_default_files_for_mode(
    mode: PetMode,
    happy_variants: &[Vec<PathBuf>],
    nomal_variants: &[Vec<PathBuf>],
    poor_condition_variants: &[Vec<PathBuf>],
    ill_variants: &[Vec<PathBuf>],
) -> Vec<PathBuf> {
    let selected = match mode {
        PetMode::Happy => flatten_variants_in_order(happy_variants),
        PetMode::Nomal => flatten_variants_in_order(nomal_variants),
        PetMode::PoorCondition => flatten_variants_in_order(poor_condition_variants),
        PetMode::Ill => flatten_variants_in_order(ill_variants),
    };

    if !selected.is_empty() {
        return selected;
    }

    let fallback_nomal = flatten_variants_in_order(nomal_variants);
    if !fallback_nomal.is_empty() {
        return fallback_nomal;
    }

    flatten_variants_in_order(happy_variants)
}
