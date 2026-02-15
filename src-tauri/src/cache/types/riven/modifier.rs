use std::{collections::HashMap, sync::LazyLock};

pub struct Modifier {
    pub good: f64,
    pub bad: f64,
}

pub static MODIFIERS: LazyLock<HashMap<String, Modifier>> = LazyLock::new(|| {
    HashMap::from([
        (
            "B2|C0".to_string(),
            Modifier {
                good: 0.99,
                bad: 0.0,
            },
        ),
        (
            "B2|C1".to_string(),
            Modifier {
                good: 1.2375,
                bad: -0.495,
            },
        ),
        (
            "B3|C0".to_string(),
            Modifier {
                good: 0.75,
                bad: 0.0,
            },
        ),
        (
            "B3|C1".to_string(),
            Modifier {
                good: 0.9375,
                bad: -0.75,
            },
        ),
    ])
});
