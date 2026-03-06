// ===== 依赖导入 =====
use std::fs;
use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use super::model::PetStats;

// ===== 存档文件路径 =====
const PET_STATS_SAVE_FILE: &str = "settings/pet_stats_save.toml";

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(default)]
struct PetStatsSave {
    stats: PetStats,
}

impl Default for PetStatsSave {
    fn default() -> Self {
        Self {
            stats: PetStats::default(),
        }
    }
}

// ===== 角色数值存档服务 =====
#[derive(Debug, Clone)]
pub struct PetStatsSaveStore {
    file_path: PathBuf,
}

impl PetStatsSaveStore {
    pub fn load() -> Self {
        Self {
            file_path: PathBuf::from(PET_STATS_SAVE_FILE),
        }
    }

    pub fn load_stats(&self) -> Option<PetStats> {
        let content = fs::read_to_string(&self.file_path).ok()?;
        let save = toml::from_str::<PetStatsSave>(&content).ok()?;
        Some(save.stats)
    }

    pub fn save_stats(&self, stats: &PetStats) -> anyhow::Result<()> {
        if let Some(parent) = self.file_path.parent() {
            fs::create_dir_all(parent)?;
        }

        let save = PetStatsSave {
            stats: stats.clone(),
        };
        let serialized = toml::to_string_pretty(&save)?;
        fs::write(&self.file_path, serialized)?;
        Ok(())
    }
}
