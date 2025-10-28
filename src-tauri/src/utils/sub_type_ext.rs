use entity::dto::SubType as EntitySubType;
use wf_market::types::SubType as WFSubType;
/// Extension trait for creating Error instances from different error types
pub trait SubTypeExt {
    fn to_entity(&self) -> Option<EntitySubType>;
    fn from_entity(entity: Option<EntitySubType>) -> Self;
}

impl SubTypeExt for WFSubType {
    fn to_entity(&self) -> Option<EntitySubType> {
        if self.is_empty() {
            return None;
        }
        Some(EntitySubType {
            variant: self.subtype.clone(),
            rank: self.rank,
            charges: self.charges,
            amber_stars: self.amber_stars,
            cyan_stars: self.cyan_stars,
        })
    }
    fn from_entity(entity: Option<EntitySubType>) -> Self {
        if entity.is_none() || entity.as_ref().unwrap().is_empty() {
            return Self::default();
        }
        let entity = entity.unwrap();
        WFSubType {
            subtype: entity.variant,
            rank: entity.rank,
            charges: entity.charges,
            amber_stars: entity.amber_stars,
            cyan_stars: entity.cyan_stars,
        }
    }
}
