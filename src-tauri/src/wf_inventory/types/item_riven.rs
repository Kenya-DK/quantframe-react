use entity::{dto::SubType, enums::RivenGrade, stock_riven::RivenAttribute};
use serde::{Deserialize, Serialize};
use utils::{get_location, Error};

use crate::{
    cache::{
        build_riven_attributes_from_fingerprint, build_riven_mod_name, compute_riven_endo_cost,
        compute_riven_kuva_cost, grade_riven, lookup_riven_multipliers, normalize_polarity,
        normalize_weapon_unique_name, CacheState,
    },
    types::ItemRivenBase,
    wf_inventory::*,
};

static COMPONENT: &str = "WFInvItemRiven";

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct WFInvItemRiven {
    #[serde(flatten)]
    pub base: ItemRivenBase,

    pub riven_type: RivenState,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum RivenState {
    Unveiled,
    Veiled,
    PreVeiled,
    Unknown,
}
impl WFInvItemRiven {
    fn new_base(state: RivenState) -> Self {
        Self {
            base: ItemRivenBase::default(),
            riven_type: state,
            ..Default::default()
        }
    }

    pub fn try_from_raw(raw: &WFInvItemRaw, cache: &CacheState) -> Result<Self, Error> {
        if !raw.is_riven() {
            return Err(Error::new(
                format!("{}:TryFromRaw", COMPONENT),
                "Item is not a riven mod",
                get_location!(),
            ));
        }

        let fingerprint = raw.get_upgrade_fingerprint();
        let state = determine_riven_state(raw);

        let mut riven = Self::new_base(state.clone());

        match state {
            RivenState::Unveiled => riven.populate_unveiled(raw, cache, &fingerprint)?,
            RivenState::PreVeiled => riven.populate_pre_veiled(raw, cache)?,
            RivenState::Veiled => riven.populate_veiled(cache, &fingerprint)?,
            RivenState::Unknown => {
                return Err(Error::new(
                    format!("{}:TryFromRaw", COMPONENT),
                    "Unable to determine riven type",
                    get_location!(),
                ))
            }
        }

        Ok(riven)
    }

    fn populate_unveiled(
        &mut self,
        raw: &WFInvItemRaw,
        cache: &CacheState,
        fingerprint: &UpgradeFingerprint,
    ) -> Result<(), Error> {
        let challenge = fingerprint.challenge.clone().ok_or_else(|| {
            Error::new(
                format!("{}:Unveiled", COMPONENT),
                "Unveiled riven missing challenge data",
                get_location!(),
            )
        })?;

        let mod_data = cache.mods().get(raw.unique_name.clone())?;
        let challenge_data = cache
            .riven()
            .get_challenge_by(challenge.challenge_type.clone())?;

        self.base.name = mod_data.name.clone();
        self.base.sub_type = Some(SubType::variant("revealed"));

        self.base.properties.set_property_value(
            "challenge_description",
            challenge_data.description.replace("|COUNT| ", ""),
        );
        self.base.properties.set_property_value(
            "challenge_description_with_complication",
            challenge_data
                .description
                .replace("|COUNT|", &challenge.required.to_string()),
        );

        Ok(())
    }
    fn populate_pre_veiled(&mut self, raw: &WFInvItemRaw, cache: &CacheState) -> Result<(), Error> {
        let mod_data = cache.mods().get(raw.unique_name.clone())?;

        self.base.name = mod_data.name.clone();
        self.base.sub_type = Some(SubType::variant("unrevealed"));

        const MSG: &str = "Riven is pre-veiled and has not been unveiled yet.";
        self.base
            .properties
            .set_property_value("challenge_description", MSG);
        self.base
            .properties
            .set_property_value("challenge_description_with_complication", MSG);

        Ok(())
    }
    fn populate_veiled(
        &mut self,
        cache: &CacheState,
        fingerprint: &UpgradeFingerprint,
    ) -> Result<(), Error> {
        let riven_cache = cache.riven();

        let weapon_key = normalize_weapon_unique_name(fingerprint.compatibility.clone());

        let weapon = riven_cache
            .get_weapon_by(&weapon_key)
            .map_err(|e| e.with_location(get_location!()))?;

        self.base.name = weapon.name.clone();
        self.base.wfm_url = weapon.wfm_url_name.clone();
        self.base.sub_type = Some(SubType::rank(fingerprint.mod_rank));
        self.base
            .properties
            .set_property_value("disposition", weapon.disposition);

        let (buffs_total, curses_total) = fingerprint.riven_stat_totals();
        let multipliers = lookup_riven_multipliers(buffs_total, curses_total)?;

        self.base.attributes = build_riven_attributes_from_fingerprint(
            &riven_cache,
            &weapon,
            fingerprint,
            multipliers,
        )?;

        sort_attributes_for_display(&mut self.base.attributes, "raw_value");

        self.base.mod_name = build_riven_mod_name(&self.base.attributes, fingerprint.buffs.len());

        sort_attributes_by_polarity(&mut self.base.attributes);

        let grade = weapon
            .god_roll
            .as_ref()
            .map(|rolls| grade_riven(rolls, &self.base.attributes, "tag").0)
            .unwrap_or(RivenGrade::Unknown);
        self.base.properties.set_property_value("grade", grade);
        self.base.polarity = normalize_polarity(fingerprint.polarity.clone());

        self.base.mastery_rank = fingerprint.mastery_rank;
        self.base.re_rolls = fingerprint.rerolls;

        let endo = compute_riven_endo_cost(
            fingerprint.mastery_rank,
            fingerprint.rerolls,
            fingerprint.mod_rank as i32,
        );
        self.base.properties.set_property_value("endo_cost", endo);

        let kuva = compute_riven_kuva_cost(fingerprint.rerolls);
        self.base.properties.set_property_value("kuva_cost", kuva);

        self.base.update_uuid();
        Ok(())
    }
}

impl Default for WFInvItemRiven {
    fn default() -> Self {
        Self {
            base: ItemRivenBase::default(),
            riven_type: RivenState::Unknown,
        }
    }
}

// ----------------------Helper Methods----------------------
fn determine_riven_state(raw: &WFInvItemRaw) -> RivenState {
    let fingerprint = raw.get_upgrade_fingerprint();

    match (fingerprint.is_riven_unveiled(), raw.id.id.is_some()) {
        (true, _) => RivenState::Unveiled,
        (false, true) => RivenState::Veiled,
        (_, false) => RivenState::PreVeiled,
    }
}
fn sort_attributes_for_display(attrs: &mut [RivenAttribute], key: impl Into<String>) {
    let key = key.into();
    attrs.sort_by(|a, b| {
        let a_value = a.properties.get_property_value(&key, 1.0);
        let b_value = b.properties.get_property_value(&key, 1.0);

        b_value
            .partial_cmp(&a_value)
            .unwrap_or(std::cmp::Ordering::Equal)
    });
}

fn sort_attributes_by_polarity(attrs: &mut [RivenAttribute]) {
    attrs.sort_by(|a, b| {
        b.positive
            .partial_cmp(&a.positive)
            .unwrap_or(std::cmp::Ordering::Equal)
    });
}
