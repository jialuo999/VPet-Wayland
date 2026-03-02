// ===== 依赖导入 =====
use std::path::{Path, PathBuf};

use crate::stats::PetMode;

use super::common::{collect_mode_variant_dirs, collect_png_files};

// ===== Shutdown 资源选择 =====
pub(crate) fn collect_shutdown_variants(shutdown_root: &Path, mode: PetMode) -> Vec<Vec<PathBuf>> {
    collect_mode_variant_dirs(shutdown_root, mode)
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
