use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct AttributeMatch {
    pub label: String,
    pub matches: bool,
}

impl AttributeMatch {
    pub fn new(label: impl Into<String>, matches: bool) -> Self {
        Self {
            label: label.into(),
            matches,
        }
    }
}

#[derive(Debug, Serialize, Default)]
pub struct RollCriteria {
    pub required: Vec<AttributeMatch>,
    pub optional: Vec<AttributeMatch>,
}

#[derive(Debug, Serialize)]
pub struct RivenRollEvaluation {
    pub valid_rolls: Vec<RollCriteria>,
    pub tolerated_negative_attributes: Vec<AttributeMatch>,
}

impl RivenRollEvaluation {
    pub fn add_negative_attribute(&mut self, label: impl Into<String>, is_tolerated: bool) {
        self.tolerated_negative_attributes.push(AttributeMatch {
            label: label.into(),
            matches: is_tolerated,
        });
    }

    pub fn add_valid_roll(&mut self, criteria: RollCriteria) {
        self.valid_rolls.push(criteria);
    }
}
impl Default for RivenRollEvaluation {
    fn default() -> Self {
        Self {
            valid_rolls: vec![],
            tolerated_negative_attributes: vec![],
        }
    }
}
