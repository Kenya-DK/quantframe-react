use std::sync::{Arc, Mutex, Weak};

use utils::*;

use crate::{cache::*, enums::*, types::*};
#[derive(Debug)]
pub struct RivenParserModule {
    client: Weak<CacheState>,

    num_buffs_attenuation: [f32; 6],
    num_curses_attenuation: [f32; 6],
}

impl RivenParserModule {
    pub const MAX_INT: i32 = 0x3FFFFFFF;                // 1073741823
    pub const SPECIFIC_FIT_ATTEN: f32 = 1.5;
    pub const BASE_DRAIN: i32 = 10;
    pub const VALUE_RANGE_MIN: f32 = 0.9;
    pub const VALUE_RANGE_MAX: f32 = 1.1;
    pub const VALUE_RANGE_SIZE: f32 =
        Self::VALUE_RANGE_MAX - Self::VALUE_RANGE_MIN;
    pub fn new(client: Arc<CacheState>) -> Arc<Self> {
        Arc::new(Self {
            client: Arc::downgrade(&client),
            num_buffs_attenuation: [0.0, 1.0, 0.66, 0.5, 0.40, 0.35],
            num_curses_attenuation: [0.0, 1.0, 0.33, 0.5, 1.25, 1.5],
        })
    }
    pub fn load(&self) -> Result<(), Error> {
    
        Ok(())
    }
    /* -------------------------------------------------------------
        Basic Conversions
    ------------------------------------------------------------- */
    pub fn get_percent_diff(mut min: f64, mut max: f64, mut value: f64) -> f64 {
        // Check if min or max is negative or zero; if so convert to positive range
        let mut is_negative_range = false;

        if min <= 0.0 || max <= 0.0 {
            min = min.abs();
            max = max.abs();
            value = value.abs();
            is_negative_range = true;
        }

        // Compute midpoint and round to 1 decimal place
        let midpoint = ((min + max) / 2.0 * 10.0).round() / 10.0;

        // Prevent division by zero
        if midpoint == 0.0 {
            return 0.0;
        }

        let percent = ((value / midpoint) - 1.0) * 100.0;

        let result = (percent * 1000.0).round() / 1000.0; // round to 3 decimals

        if is_negative_range {
            -result
        } else {
            result
        }
    }
    
    pub fn riven_int_to_float(&self, int_value: i32) -> f32 {
        let f = int_value as f32 / Self::MAX_INT as f32;
        if f >= 0.0 && f <= 1.0 { f } else { 0.0 }
    }

    pub fn float_to_riven_int(&self, value: f32) -> i32 {
        (value * Self::MAX_INT as f32).round() as i32
    }

    pub fn lerp(&self, a: f32, b: f32, t: f32) -> f32 {
        a + (b - a) * t
    }

    pub fn float_to_left_value(&self, val: f32) -> i32 {
        (val * 2000.0 + 9000.0).round() as i32
    }

    pub fn compute_attenuation(&self, omega_attenuation: f32) -> f32 {
        Self::SPECIFIC_FIT_ATTEN * omega_attenuation * Self::BASE_DRAIN as f32
    }
    
    pub fn normalize_roll_value(raw_value: f32) -> f32 {
        (raw_value - Self::VALUE_RANGE_MIN) / Self::VALUE_RANGE_SIZE
    }
    
    /* -------------------------------------------------------------
        Display Value Transformations
    ------------------------------------------------------------- */
    pub fn value_to_display_value(tag: &str, value: f64) -> f64 {
        const TWO_DIGIT_TAGS: &[&str] = &[
            "WeaponFactionDamageGrineer",
            "WeaponFactionDamageCorpus",
            "WeaponFactionDamageInfested",
            "WeaponMeleeFactionDamageGrineer",
            "WeaponMeleeFactionDamageCorpus",
            "WeaponMeleeFactionDamageInfested",
        ];

        const ONE_DECIMAL_TAGS: &[&str] = &[
            "WeaponMeleeComboInitialBonusMod",
            "ComboDurationMod",
            "WeaponMeleeRangeIncMod",
        ];

        if TWO_DIGIT_TAGS.contains(&tag) {
            return (value * 100.0).round() / 100.0;
        }

        if ONE_DECIMAL_TAGS.contains(&tag) {
            return (value * 10.0).round() / 10.0;
        }

        // Matches JS: Math.round(value * 1000) / 10
        (value * 1000.0).round() / 10.0
    }

    pub fn display_value_to_value(tag: &str, display_value: f64) -> f64 {
        const PASSTHROUGH_TAGS: &[&str] = &[
            "WeaponFactionDamageGrineer",
            "WeaponFactionDamageCorpus",
            "WeaponFactionDamageInfested",
            "WeaponMeleeFactionDamageGrineer",
            "WeaponMeleeFactionDamageCorpus",
            "WeaponMeleeFactionDamageInfested",
            "WeaponMeleeComboInitialBonusMod",
            "ComboDurationMod",
            "WeaponMeleeRangeIncMod",
        ];

        if PASSTHROUGH_TAGS.contains(&tag) {
            display_value
        } else {
            display_value / 100.0
        }
    }
    
    /* -------------------------------------------------------------
        Grading
    ------------------------------------------------------------- */
    pub fn roll_to_grade(&self, value: f32) -> &'static str {
        let grade_value = self.lerp(-10.0, 10.0, value);

        if grade_value < -11.5 || grade_value > 11.5 {
            return "X";
        }

        if grade_value >= 9.5  { return "S";  }
        if grade_value >= 7.5  { return "A+"; }
        if grade_value >= 5.5  { return "A";  }
        if grade_value >= 3.5  { return "A-"; }
        if grade_value >= 1.5  { return "B+"; }
        if grade_value >= -1.5 { return "B";  }
        if grade_value >= -3.5 { return "B-"; }
        if grade_value >= -5.5 { return "C+"; }
        if grade_value >= -7.5 { return "C";  }
        if grade_value >= -9.5 { return "C-"; }

        "F"
    }
    
    pub fn grade_to_score(grade: &str) -> i32 {
        match grade {
            "S"  => 100,
            "A+" => 95,
            "A"  => 90,
            "A-" => 85,
            "B+" => 80,
            "B"  => 70,
            "B-" => 60,
            "C+" => 50,
            "C"  => 40,
            "C-" => 30,
            "F"  => 10,
            _    => 0, // For "X" or unknown
        }
    }
    /**
     * Creates a new `RivenParserModule` from an existing one, sharing the client.
     * This is useful for cloning modules when the client state changes.
     */
    pub fn from_existing(old: &RivenParserModule, client: Arc<CacheState>) -> Arc<Self> {
        Arc::new(Self {
            client: Arc::downgrade(&client),
            num_buffs_attenuation: old.num_buffs_attenuation.clone(),
            num_curses_attenuation: old.num_curses_attenuation.clone(),
        })
    }
}
