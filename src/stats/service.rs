// ===== 依赖导入 =====
use std::cell::{Ref, RefCell};
use std::rc::Rc;

use crate::config::PanelDebugConfig;
use rand::random;

use super::food::FoodItem;
use super::model::{InteractType, PetMode, PetStats};

// ===== 系统参数常量 =====
const LOGIC_INTERVAL_MIN_SECS: f64 = 5.0;
const LOGIC_INTERVAL_MAX_SECS: f64 = 60.0;

const DECAY_BASE: f64 = 1.0;
const HEALTH_MAX: f64 = 100.0;

const DECAY_BALANCE_FOOD_DRINK: f64 = 1.0;
const DECAY_BALANCE_STRENGTH: f64 = 0.8;
const DECAY_BALANCE_FEELING: f64 = 0.5;
const DECAY_BALANCE_HEALTH: f64 = 0.3;
const FOOD_AUTO_CONSUME_HIGH_RATIO: f64 = 0.5;
const FOOD_HEALTH_RISK_LOW_RATIO: f64 = 0.25;
const AUTO_CONSUME_FOOD_TO_STRENGTH_RATE: f64 = 1.0;
const FEELING_DROP_IDLE_MULTIPLIER_CAP: f64 = 3.0;
const FEELING_DROP_IDLE_REF_SECS: f64 = 60.0;
const INTERACT_MIN_STRENGTH_REQUIRED: f64 = 10.0;
const INTERACT_STRENGTH_COST: f64 = 2.0;
const INTERACT_FEELING_GAIN: f64 = 1.0;

// ===== 面板显示上限 =====
#[derive(Debug, Clone, Copy)]
struct PanelLimits {
    basic_stat_max: f64,
    experience_max: f64,
    level_max: f64,
}

impl Default for PanelLimits {
    fn default() -> Self {
        Self {
            basic_stat_max: 100.0,
            experience_max: 100.0,
            level_max: 100.0,
        }
    }
}

// ===== 宠物数值服务 =====
#[derive(Debug, Clone)]
pub struct PetStatsService {
    stats: Rc<RefCell<PetStats>>,
    limits: Rc<RefCell<PanelLimits>>,
    secs_since_last_interact: Rc<RefCell<f64>>,
    logic_interval_secs: f64,
}

impl Default for PetStatsService {
    fn default() -> Self {
        Self::new(PetStats::default(), LOGIC_INTERVAL_MIN_SECS)
    }
}

impl PetStatsService {
	// 构造服务并限制逻辑间隔范围
    pub fn new(initial_stats: PetStats, logic_interval_secs: f64) -> Self {
        Self {
            stats: Rc::new(RefCell::new(initial_stats)),
            limits: Rc::new(RefCell::new(PanelLimits::default())),
            secs_since_last_interact: Rc::new(RefCell::new(0.0)),
            logic_interval_secs: clamp_logic_interval(logic_interval_secs),
        }
    }

    pub fn from_panel_config(panel_config: PanelDebugConfig, logic_interval_secs: f64) -> Self {
        let service = Self::new(panel_config_to_stats(&panel_config), logic_interval_secs);
        service.apply_panel_config(panel_config);
        service
    }

    #[allow(dead_code)]
    pub fn from_shared(stats: Rc<RefCell<PetStats>>, logic_interval_secs: f64) -> Self {
        Self {
            stats,
            limits: Rc::new(RefCell::new(PanelLimits::default())),
            secs_since_last_interact: Rc::new(RefCell::new(0.0)),
            logic_interval_secs: clamp_logic_interval(logic_interval_secs),
        }
    }

    #[allow(dead_code)]
    pub fn stats(&self) -> Ref<'_, PetStats> {
        self.stats.borrow()
    }

    pub fn get_stats(&self) -> PetStats {
        self.stats.borrow().clone()
    }

    pub fn replace_stats(&self, next_stats: PetStats) {
        *self.stats.borrow_mut() = next_stats;
    }

	// 每次配置更新都重建数值与显示上限
    pub fn apply_panel_config(&self, panel_config: PanelDebugConfig) {
        {
            let mut limits = self.limits.borrow_mut();
            limits.basic_stat_max = panel_config.basic_stat_max as f64;
            limits.experience_max = panel_config.experience_max as f64;
            limits.level_max = panel_config.level_max as f64;
        }
        self.replace_stats(panel_config_to_stats(&panel_config));
    }

    pub fn basic_stat_max(&self) -> f64 {
        self.limits.borrow().basic_stat_max
    }

    pub fn experience_max(&self) -> f64 {
        self.limits.borrow().experience_max
    }

    pub fn level_max(&self) -> f64 {
        self.limits.borrow().level_max
    }

    #[allow(dead_code)]
    pub fn shared_stats(&self) -> Rc<RefCell<PetStats>> {
        self.stats.clone()
    }

    #[allow(dead_code)]
    pub fn logic_interval_secs(&self) -> f64 {
        self.logic_interval_secs
    }

    #[allow(dead_code)]
    pub fn set_logic_interval_secs(&mut self, logic_interval_secs: f64) {
        self.logic_interval_secs = clamp_logic_interval(logic_interval_secs);
    }

	// 逻辑 tick：处理自然衰减与升级
    pub fn on_tick(&mut self, delta_secs: f64) {
        if delta_secs <= 0.0 {
            return;
        }

        let scale = (delta_secs / self.logic_interval_secs) * DECAY_BASE;

        {
            let mut secs_since_last_interact = self.secs_since_last_interact.borrow_mut();
            *secs_since_last_interact += delta_secs;
        }

        let secs_since_last_interact = *self.secs_since_last_interact.borrow();
        let idle_multiplier =
            (1.0 + secs_since_last_interact / FEELING_DROP_IDLE_REF_SECS)
                .min(FEELING_DROP_IDLE_MULTIPLIER_CAP);
        let freedrop = DECAY_BALANCE_FEELING * scale * idle_multiplier;

        let basic_stat_max = self.basic_stat_max();
        let mut stats = self.stats.borrow_mut();
        let food_half_threshold = basic_stat_max * FOOD_AUTO_CONSUME_HIGH_RATIO;
        let food_quarter_threshold = basic_stat_max * FOOD_HEALTH_RISK_LOW_RATIO;

        stats.strength_food -= DECAY_BALANCE_FOOD_DRINK * scale;
        stats.strength_drink -= DECAY_BALANCE_FOOD_DRINK * scale;
        stats.strength -= DECAY_BALANCE_STRENGTH * scale;

        if stats.strength_food >= food_half_threshold {
            stats.strength_food -= AUTO_CONSUME_FOOD_TO_STRENGTH_RATE * scale;
            stats.strength += AUTO_CONSUME_FOOD_TO_STRENGTH_RATE * scale;
        } else if stats.strength_food <= food_quarter_threshold {
            stats.health -= random::<f64>() * scale;
        }

        let raw_feeling = stats.feeling - freedrop;
        stats.feeling = raw_feeling;

        if raw_feeling < 20.0 && stats.strength_food < 10.0 && stats.strength_drink < 10.0 {
            stats.health -= DECAY_BALANCE_HEALTH * scale;
        }
        drop(stats);

        if raw_feeling < 0.0 {
            self.apply_likability_gain(raw_feeling / 2.0);
        }

        let mut stats = self.stats.borrow_mut();

        clamp_stats(&mut stats, basic_stat_max);
        apply_level_up_if_needed(&mut stats);
        clamp_stats(&mut stats, basic_stat_max);
    }

	// 投喂：恢复基础属性并增加好感
    #[allow(dead_code)]
    pub fn on_feed(&mut self, food: &FoodItem) {
        let basic_stat_max = self.basic_stat_max();

        self.apply_likability_gain(food.likability);

        let bonus = {
            let stats = self.stats.borrow();
            likability_bonus(stats.likability)
        };
        let recover_factor = 1.0 + bonus;

        self.apply_feeling_gain(food.feeling * recover_factor);

        let mut stats = self.stats.borrow_mut();
        stats.strength_food += food.strength_food * recover_factor;
        stats.strength_drink += food.strength_drink * recover_factor;

        clamp_stats(&mut stats, basic_stat_max);
    }

	// 互动：消耗体力并变化心情/经验
    #[allow(dead_code)]
    pub fn on_interact(&mut self, _interact_type: InteractType) -> bool {
        let basic_stat_max = self.basic_stat_max();

        let (can_animate, should_apply_effect) = {
            let stats = self.stats.borrow();
            let has_enough_strength = stats.strength >= INTERACT_MIN_STRENGTH_REQUIRED;
            let feeling_not_full = stats.feeling < stats.feeling_max;
            (has_enough_strength, has_enough_strength && feeling_not_full)
        };

        if !can_animate {
            return false;
        }

        *self.secs_since_last_interact.borrow_mut() = 0.0;

        if !should_apply_effect {
            return true;
        }

        {
            let mut stats = self.stats.borrow_mut();
            stats.strength =
                (stats.strength - INTERACT_STRENGTH_COST).clamp(0.0, stats.strength_max);
            let level_f = stats.level as f64;
            stats.exp += 1.0 * level_f;
        }

        self.apply_feeling_gain(INTERACT_FEELING_GAIN);

        let mut stats = self.stats.borrow_mut();

        clamp_stats(&mut stats, basic_stat_max);
        apply_level_up_if_needed(&mut stats);
        clamp_stats(&mut stats, basic_stat_max);

        true
    }

	// 心情变化统一入口（并联动好感）
    fn apply_feeling_gain(&mut self, feeling_gain: f64) {
        let raw_feeling = {
            let stats = self.stats.borrow();
            stats.feeling + feeling_gain
        };

        self.apply_likability_gain(feeling_gain);

        let mut stats = self.stats.borrow_mut();
        stats.feeling = raw_feeling.clamp(0.0, stats.feeling_max);
    }

	// 好感变化统一入口（溢出转为健康）
    fn apply_likability_gain(&mut self, delta: f64) {
        let mut stats = self.stats.borrow_mut();
        let new_val = stats.likability + delta;
        if new_val > stats.likability_max {
            let overflow = new_val - stats.likability_max;
            stats.likability = stats.likability_max;
            stats.health = (stats.health + overflow).min(100.0);
        } else {
            stats.likability = new_val.clamp(0.0, stats.likability_max);
        }
    }

    pub fn cal_mode(&self) -> PetMode {
        self.stats.borrow().cal_mode()
    }
}

// ===== 私有辅助函数 =====
fn apply_level_up_if_needed(stats: &mut PetStats) {
    loop {
        let needed = stats.level_up_exp_needed();
        if stats.exp < needed {
            break;
        }

        stats.exp -= needed;
        stats.level = stats.level.saturating_add(1);
        stats.feeling_max = PetStats::feeling_max_for_level(stats.level);
        stats.likability_max = PetStats::likability_max_for_level(stats.level);
        stats.strength_max = PetStats::strength_max_for_level(stats.level);
    }
}

fn clamp_stats(stats: &mut PetStats, basic_stat_max: f64) {
    let base_max = basic_stat_max.max(1.0);

    stats.health = stats.health.clamp(0.0, HEALTH_MAX);
    stats.feeling = stats.feeling.clamp(0.0, stats.feeling_max.max(0.0));
    stats.strength = stats.strength.clamp(0.0, stats.strength_max.max(0.0));
    stats.strength_food = stats.strength_food.clamp(0.0, base_max);
    stats.strength_drink = stats.strength_drink.clamp(0.0, base_max);
    stats.likability = stats.likability.clamp(0.0, stats.likability_max.max(0.0));
    stats.exp = stats.exp.max(0.0);
}

fn panel_config_to_stats(panel_config: &PanelDebugConfig) -> PetStats {
    let level = panel_config.default_level.max(1);
    let basic_stat_max = panel_config.basic_stat_max as f64;

    PetStats {
        health: panel_config.default_health as f64,
        feeling: panel_config.default_mood as f64,
        feeling_max: basic_stat_max,
        likability_max: PetStats::likability_max_for_level(level),
        strength: panel_config.default_stamina as f64,
        strength_max: basic_stat_max,
        strength_food: panel_config.default_satiety as f64,
        strength_drink: panel_config.default_thirst as f64,
        likability: panel_config.default_affinity as f64,
        level,
        exp: panel_config.default_experience as f64,
    }
}

#[allow(dead_code)]
fn likability_bonus(likability: f64) -> f64 {
    if likability >= 80.0 {
        0.20
    } else if likability >= 40.0 {
        0.10
    } else {
        0.0
    }
}

fn clamp_logic_interval(value: f64) -> f64 {
    value.clamp(LOGIC_INTERVAL_MIN_SECS, LOGIC_INTERVAL_MAX_SECS)
}
