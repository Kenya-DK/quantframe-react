use std::{
    collections::HashMap,
    path::PathBuf,
    sync::{Arc, Mutex},
};

use tract_onnx::prelude::*;
use utils::{
    get_location, info, read_json_file, read_json_file_optional, warning, Error, LoggerOptions,
};

use crate::cache::*;

const MASK: &str = "<none>";

type TractModel = SimplePlan<TypedFact, Box<dyn TypedOp>, Graph<TypedFact, Box<dyn TypedOp>>>;

/// Loads the riven price ONNX model and runs price predictions.
#[derive(Debug)]
pub struct RivenPricerModule {
    path: PathBuf,
    model: Mutex<Option<TractModel>>,
    weapon_vocab: Mutex<HashMap<String, i32>>,
    attr_vocab: Mutex<HashMap<String, i32>>,
    effect_to_url: Mutex<HashMap<String, String>>,
    mask_index: Mutex<i32>,
}

impl RivenPricerModule {
    pub fn new(client: Arc<CacheState>) -> Arc<Self> {
        Arc::new(Self {
            path: client.base_path.join("models/price_model.onnx"),
            model: Mutex::new(None),
            weapon_vocab: Mutex::new(HashMap::new()),
            attr_vocab: Mutex::new(HashMap::new()),
            effect_to_url: Mutex::new(HashMap::new()),
            mask_index: Mutex::new(0),
        })
    }

    /// Non-fatal load used during cache initialization: a missing or invalid
    /// model simply leaves the module unavailable instead of failing the cache.
    pub fn load(&self) -> Result<(), Error> {
        if let Err(e) = self.try_load() {
            warning(
                "Cache:RivenPricer:load",
                format!("Price model not loaded: {e}"),
                &LoggerOptions::default(),
            );
            *self.model.lock().unwrap() = None;
        }
        Ok(())
    }

    /// Reloads the model from disk, returning an error if it cannot be loaded.
    pub fn reload(&self) -> Result<(), Error> {
        self.try_load()
    }

    fn try_load(&self) -> Result<(), Error> {
        if !self.path.exists() {
            return Err(Error::new(
                "Cache:RivenPricer:load",
                format!("Price model not found at {}", self.path.display()),
                get_location!(),
            ));
        }

        let weapon_vocab_path = self.vocab_path("weapon_vocab.json");
        let attr_vocab_path = self.vocab_path("attr_vocab.json");
        if !weapon_vocab_path.exists() || !attr_vocab_path.exists() {
            return Err(Error::new(
                "Cache:RivenPricer:load",
                format!(
                    "Price model vocabularies missing next to {}",
                    self.path.display()
                ),
                get_location!(),
            ));
        }

        let model = self.load_model()?;

        let weapon_vocab_list: Vec<String> = read_json_file(&weapon_vocab_path)?;
        let attr_vocab_list: Vec<String> = read_json_file(&attr_vocab_path)?;
        let effect_to_url: HashMap<String, String> =
            read_json_file_optional(&self.vocab_path("effect_to_url_name.json"))
                .unwrap_or_default();

        let weapon_vocab: HashMap<String, i32> = weapon_vocab_list
            .into_iter()
            .enumerate()
            .map(|(i, s)| (s.to_lowercase(), i as i32))
            .collect();
        let attr_vocab: HashMap<String, i32> = attr_vocab_list
            .into_iter()
            .enumerate()
            .map(|(i, s)| (s.to_lowercase(), i as i32))
            .collect();

        let weapon_count = weapon_vocab.len();
        *self.mask_index.lock().unwrap() = *attr_vocab.get(MASK).unwrap_or(&0);
        *self.weapon_vocab.lock().unwrap() = weapon_vocab;
        *self.attr_vocab.lock().unwrap() = attr_vocab;
        *self.effect_to_url.lock().unwrap() = effect_to_url
            .into_iter()
            .map(|(k, v)| (k.to_lowercase(), v))
            .collect();
        *self.model.lock().unwrap() = Some(model);

        info(
            "Cache:RivenPricer:load",
            format!("Loaded price model ({} weapons)", weapon_count),
            &LoggerOptions::default(),
        );
        Ok(())
    }

    fn vocab_path(&self, name: &str) -> PathBuf {
        self.path
            .parent()
            .map(|parent| parent.join(name))
            .unwrap_or_else(|| PathBuf::from(name))
    }

    fn load_model(&self) -> Result<TractModel, Error> {
        tract_onnx::onnx()
            .model_for_path(&self.path)
            .and_then(|m| {
                m.with_input_fact(0, InferenceFact::dt_shape(i32::datum_type(), tvec![1, 1]))
            })
            .and_then(|m| {
                m.with_input_fact(1, InferenceFact::dt_shape(f32::datum_type(), tvec![1, 1]))
            })
            .and_then(|m| {
                m.with_input_fact(2, InferenceFact::dt_shape(i32::datum_type(), tvec![1, 4]))
            })
            .and_then(|m| m.into_optimized())
            .and_then(|m| m.into_runnable())
            .map_err(|e| {
                Error::new(
                    "Cache:RivenPricer:LoadModel",
                    format!("Failed to load price_model.onnx: {e:?}"),
                    get_location!(),
                )
            })
    }

    fn ensure_loaded(&self) {
        if self.is_loaded() || !self.is_available() {
            return;
        }
        if let Err(e) = self.try_load() {
            e.log("riven_pricer_load.log");
        }
    }

    /// Returns true when `price_model.onnx` is present on disk.
    pub fn is_available(&self) -> bool {
        self.path.exists()
    }

    /// Returns true when the model is currently loaded and ready to predict.
    pub fn is_loaded(&self) -> bool {
        self.model
            .lock()
            .map(|model| model.is_some())
            .unwrap_or(false)
    }

    fn resolve_attr(&self, value: &str) -> i32 {
        let key = value.trim().to_lowercase();
        let attr_vocab = self.attr_vocab.lock().unwrap();
        if let Some(idx) = attr_vocab.get(&key) {
            return *idx;
        }
        if let Some(slug) = self.effect_to_url.lock().unwrap().get(&key) {
            if let Some(idx) = attr_vocab.get(&slug.to_lowercase()) {
                return *idx;
            }
        }
        *self.mask_index.lock().unwrap()
    }

    fn resolve_weapon(&self, value: &str) -> Option<i32> {
        let key = value.trim().to_lowercase();
        let weapon_vocab = self.weapon_vocab.lock().unwrap();
        if let Some(idx) = weapon_vocab.get(&key) {
            return Some(*idx);
        }
        // Fall back to a suffix match (e.g. name vs slug differences).
        weapon_vocab
            .iter()
            .find(|(slug, _)| slug.ends_with(&key) || key.ends_with(slug.as_str()))
            .map(|(_, idx)| *idx)
    }

    /// Runs a price prediction. Returns `None` if the model is unavailable or
    /// the weapon is unknown.
    pub fn predict(&self, input: &RivenPriceInput) -> Option<RivenPriceEstimate> {
        self.ensure_loaded();

        let weapon_idx = self.resolve_weapon(&input.weapon)?;

        let p1 = input.positives.get(0).map(|v| self.resolve_attr(v));
        let p2 = input.positives.get(1).map(|v| self.resolve_attr(v));
        let p3 = input.positives.get(2).map(|v| self.resolve_attr(v));
        let neg = input.negative.as_deref().map(|v| self.resolve_attr(v));

        let mask_index = *self.mask_index.lock().unwrap();
        let attr_indices = [
            p1.unwrap_or(mask_index),
            p2.unwrap_or(mask_index),
            p3.unwrap_or(mask_index),
            neg.unwrap_or(mask_index),
        ];

        let re_rolled: f32 = if input.re_rolls > 0 { 1.0 } else { 0.0 };

        let weapon_tensor: Tensor = tract_ndarray::arr2(&[[weapon_idx]]).into_tensor();
        let re_rolled_tensor: Tensor = tract_ndarray::arr2(&[[re_rolled]]).into_tensor();
        let attr_tensor: Tensor = tract_ndarray::arr2(&[[
            attr_indices[0],
            attr_indices[1],
            attr_indices[2],
            attr_indices[3],
        ]])
        .into_tensor();

        let model = self.model.lock().ok()?;
        let model = model.as_ref()?;
        let outputs = model
            .run(tvec![
                weapon_tensor.into(),
                re_rolled_tensor.into(),
                attr_tensor.into(),
            ])
            .ok()?;

        let output = outputs[0].to_array_view::<f32>().ok()?;
        let log_price = output[[0, 0]];
        let price = (log_price.exp() - 1.0).max(1.0);

        Some(RivenPriceEstimate {
            price,
            log_price,
            weapon_idx,
            attr_indices,
        })
    }

    /// The weapon keys known to the model vocabulary.
    pub fn get_weapon_names(&self) -> Vec<String> {
        self.weapon_vocab
            .lock()
            .map(|vocab| vocab.keys().cloned().collect())
            .unwrap_or_default()
    }
}
