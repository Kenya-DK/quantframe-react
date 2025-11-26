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
    /* -------------------------------------------------------------
        Basic Conversions
    ------------------------------------------------------------- */
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
