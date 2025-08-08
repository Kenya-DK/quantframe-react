use std::{
    fs::File,
    io::{Read, Write},
    path::PathBuf,
    sync::{Arc, OnceLock, RwLock},
};

use qf_api::Client as QFClient;
use utils::{get_location, info, Error, LoggerOptions};
use wf_market::endpoints::user;

use crate::{
    app::User,
    cache::types::CacheVersion,
    emit_startup, helper,
    utils::modules::states::{self, ErrorFromExt},
};

use super::modules::*;

#[derive(Clone, Debug)]
pub struct CacheState {
    self_arc: OnceLock<Arc<CacheState>>,
    pub base_path: PathBuf,
    pub version: CacheVersion,
    // Modules
    all_items_module: OnceLock<Arc<AllItemsModule>>,
    arcane_module: OnceLock<Arc<ArcaneModule>>,
    archgun_module: OnceLock<Arc<ArchGunModule>>,
    archmelee_module: OnceLock<Arc<ArchMeleeModule>>,
    archwing_module: OnceLock<Arc<ArchwingModule>>,
    fish_module: OnceLock<Arc<FishModule>>,
    melee_module: OnceLock<Arc<MeleeModule>>,
    misc_module: OnceLock<Arc<MiscModule>>,
    mod_module: OnceLock<Arc<ModModule>>,
    pet_module: OnceLock<Arc<PetModule>>,
    primary_module: OnceLock<Arc<PrimaryModule>>,
    relics_module: OnceLock<Arc<RelicsModule>>,
    resource_module: OnceLock<Arc<ResourceModule>>,
    riven_module: OnceLock<Arc<RivenModule>>,
    secondary_module: OnceLock<Arc<SecondaryModule>>,
    sentinel_module: OnceLock<Arc<SentinelModule>>,
    skin_module: OnceLock<Arc<SkinModule>>,
    tradable_item_module: OnceLock<Arc<TradableItemModule>>,
    warframe_module: OnceLock<Arc<WarframeModule>>,
    item_price_module: OnceLock<Arc<ItemPriceModule>>,
    theme_module: OnceLock<Arc<ThemeModule>>,
}

impl CacheState {
    fn arc(&self) -> Arc<Self> {
        self.self_arc
            .get_or_init(|| {
                Arc::new(Self {
                    self_arc: OnceLock::new(),
                    base_path: self.base_path.clone(),
                    version: self.version.clone(),
                    // Initialize modules
                    all_items_module: self.all_items_module.clone(),
                    arcane_module: self.arcane_module.clone(),
                    archgun_module: self.archgun_module.clone(),
                    archmelee_module: self.archmelee_module.clone(),
                    archwing_module: self.archwing_module.clone(),
                    fish_module: self.fish_module.clone(),
                    melee_module: self.melee_module.clone(),
                    misc_module: self.misc_module.clone(),
                    mod_module: self.mod_module.clone(),
                    pet_module: self.pet_module.clone(),
                    primary_module: self.primary_module.clone(),
                    relics_module: self.relics_module.clone(),
                    resource_module: self.resource_module.clone(),
                    riven_module: self.riven_module.clone(),
                    secondary_module: self.secondary_module.clone(),
                    sentinel_module: self.sentinel_module.clone(),
                    skin_module: self.skin_module.clone(),
                    tradable_item_module: self.tradable_item_module.clone(),
                    warframe_module: self.warframe_module.clone(),
                    item_price_module: self.item_price_module.clone(),
                    theme_module: self.theme_module.clone(),
                })
            })
            .clone()
    }

    pub async fn new(qf_client: &QFClient, user: &User) -> Result<Self, Error> {
        let version =
            CacheVersion::load().expect("Failed to load cache version from cache_version.json");

        emit_startup!("cache.initializing", json!({}));
        let mut client = CacheState {
            self_arc: OnceLock::new(),
            base_path: helper::get_app_storage_path().join("cache"),
            version,
            all_items_module: OnceLock::new(),
            arcane_module: OnceLock::new(),
            archgun_module: OnceLock::new(),
            archmelee_module: OnceLock::new(),
            archwing_module: OnceLock::new(),
            fish_module: OnceLock::new(),
            melee_module: OnceLock::new(),
            misc_module: OnceLock::new(),
            mod_module: OnceLock::new(),
            pet_module: OnceLock::new(),
            primary_module: OnceLock::new(),
            relics_module: OnceLock::new(),
            resource_module: OnceLock::new(),
            riven_module: OnceLock::new(),
            secondary_module: OnceLock::new(),
            sentinel_module: OnceLock::new(),
            skin_module: OnceLock::new(),
            tradable_item_module: OnceLock::new(),
            warframe_module: OnceLock::new(),
            item_price_module: OnceLock::new(),
            theme_module: OnceLock::new(),
        };
        if !user.verification || !user.qf_banned || !user.wfm_banned {
            return Ok(client);
        }
        match client.load(qf_client).await {
            Ok((cache_version_id, price_version_id)) => {
                client.version.id = cache_version_id;
                client.version.id_price = price_version_id;
                client.version.save()?;
                info(
                    "Cache:Version",
                    "Cache loaded successfully.",
                    LoggerOptions::default(),
                );
            }
            Err(e) => return Err(e.with_location(get_location!())),
        }
        Ok(client)
    }

    async fn check_update(&self, qf_client: &QFClient) -> Result<(bool, String), Error> {
        let current_version = self.version.id.clone();
        let remote_version = match qf_client.cache().get_cache_id().await {
            Ok(id) => id,
            Err(e) => {
                let err = Error::from_qf(
                    "Cache:CheckUpdate",
                    "Failed to get cache ID",
                    e,
                    get_location!(),
                );
                return Err(err);
            }
        };
        Ok((current_version != remote_version, remote_version))
    }

    pub async fn load(&self, qf_client: &QFClient) -> Result<(String, String), Error> {
        let (cache_require_update, cache_version_id) = self.check_update(qf_client).await?;
        let (price_require_update, price_version_id) =
            self.item_price().check_update(qf_client).await?;

        if cache_require_update {
            emit_startup!("cache.updating", json!({}));
            match self.extract(qf_client).await {
                Ok(()) => {
                    info(
                        "Cache:Load",
                        "Cache updated successfully.",
                        LoggerOptions::default(),
                    );
                }
                Err(e) => return Err(e.with_location(get_location!())),
            }
        }

        // Update Item Prices if user is verified
        self.item_price()
            .load(qf_client, price_require_update)
            .await?;
        self.arcane().load()?;
        self.archgun().load()?;
        self.archmelee().load()?;
        self.archwing().load()?;
        self.fish().load()?;
        self.melee().load()?;
        self.misc().load()?;
        self.mod_module().load()?;
        self.pet().load()?;
        self.primary().load()?;
        self.relics().load()?;
        self.resource().load()?;
        self.riven().load()?;
        self.secondary().load()?;
        self.sentinel().load()?;
        self.skin().load()?;
        self.tradable_item().load()?;
        self.warframe().load()?;
        self.all_items().load()?;
        self.theme().load()?;
        Ok((cache_version_id, price_version_id))
    }

    async fn extract(&self, qf_client: &QFClient) -> Result<(), Error> {
        let zip_data =
            qf_client.cache().download_cache().await.map_err(|e| {
                Error::from_qf("Cache", "Failed to download cache", e, get_location!())
            })?;

        let reader = std::io::Cursor::new(zip_data);
        let mut archive = zip::ZipArchive::new(reader).map_err(|e| {
            Error::from_zip(
                "Cache",
                "cache.zip",
                "Failed to read cache zip",
                e,
                get_location!(),
            )
        })?;

        let extract_to = helper::get_app_storage_path().join("cache");
        let mut total_size = 0u64;
        for i in 0..archive.len() {
            let mut file = archive.by_index(i).map_err(|e| {
                Error::from_zip(
                    "Cache:Extract",
                    &format!("cache.zip[{}]", i),
                    "Failed to read file from cache zip",
                    e,
                    get_location!(),
                )
            })?;
            let output_path = extract_to.join(file.mangled_name());

            if file.is_dir() {
                std::fs::create_dir_all(&output_path).map_err(|e| {
                    Error::from_io(
                        "Cache",
                        &output_path,
                        "Failed to create directory for cache file",
                        e,
                        get_location!(),
                    )
                })?;
            } else {
                if let Some(parent) = output_path.parent() {
                    if !parent.exists() {
                        std::fs::create_dir_all(parent).map_err(|e| {
                            Error::from_io(
                                "Cache",
                                &parent.to_path_buf(),
                                "Failed to create parent directory for cache file",
                                e,
                                get_location!(),
                            )
                        })?;
                    }
                }

                let mut output_file = File::create(&output_path).map_err(|e| {
                    Error::from_io(
                        "Cache",
                        &output_path,
                        "Failed to create cache file",
                        e,
                        get_location!(),
                    )
                })?;
                total_size += file.size();
                std::io::copy(&mut file, &mut output_file).map_err(|e| {
                    Error::from_io(
                        "Cache",
                        &output_path,
                        "Failed to write cache file",
                        e,
                        get_location!(),
                    )
                })?;
            }
        }

        // Implement the logic to extract the cache
        info(
            "Cache:Extract",
            format!("Extracting cache... ({} bytes)", total_size),
            LoggerOptions::default(),
        );
        Ok(())
    }

    // Modules
    pub fn item_price(&self) -> Arc<ItemPriceModule> {
        self.item_price_module
            .get_or_init(|| ItemPriceModule::new(self.arc()))
            .clone()
    }
    pub fn all_items(&self) -> Arc<AllItemsModule> {
        self.all_items_module
            .get_or_init(|| AllItemsModule::new(self.arc()))
            .clone()
    }
    pub fn arcane(&self) -> Arc<ArcaneModule> {
        self.arcane_module
            .get_or_init(|| ArcaneModule::new(self.arc()))
            .clone()
    }
    pub fn archgun(&self) -> Arc<ArchGunModule> {
        self.archgun_module
            .get_or_init(|| ArchGunModule::new(self.arc()))
            .clone()
    }
    pub fn archmelee(&self) -> Arc<ArchMeleeModule> {
        self.archmelee_module
            .get_or_init(|| ArchMeleeModule::new(self.arc()))
            .clone()
    }
    pub fn archwing(&self) -> Arc<ArchwingModule> {
        self.archwing_module
            .get_or_init(|| ArchwingModule::new(self.arc()))
            .clone()
    }
    pub fn fish(&self) -> Arc<FishModule> {
        self.fish_module
            .get_or_init(|| FishModule::new(self.arc()))
            .clone()
    }
    pub fn melee(&self) -> Arc<MeleeModule> {
        self.melee_module
            .get_or_init(|| MeleeModule::new(self.arc()))
            .clone()
    }
    pub fn misc(&self) -> Arc<MiscModule> {
        self.misc_module
            .get_or_init(|| MiscModule::new(self.arc()))
            .clone()
    }
    pub fn mod_module(&self) -> Arc<ModModule> {
        self.mod_module
            .get_or_init(|| ModModule::new(self.arc()))
            .clone()
    }
    pub fn pet(&self) -> Arc<PetModule> {
        self.pet_module
            .get_or_init(|| PetModule::new(self.arc()))
            .clone()
    }
    pub fn primary(&self) -> Arc<PrimaryModule> {
        self.primary_module
            .get_or_init(|| PrimaryModule::new(self.arc()))
            .clone()
    }
    pub fn relics(&self) -> Arc<RelicsModule> {
        self.relics_module
            .get_or_init(|| RelicsModule::new(self.arc()))
            .clone()
    }
    pub fn resource(&self) -> Arc<ResourceModule> {
        self.resource_module
            .get_or_init(|| ResourceModule::new(self.arc()))
            .clone()
    }
    pub fn riven(&self) -> Arc<RivenModule> {
        self.riven_module
            .get_or_init(|| RivenModule::new(self.arc()))
            .clone()
    }
    pub fn secondary(&self) -> Arc<SecondaryModule> {
        self.secondary_module
            .get_or_init(|| SecondaryModule::new(self.arc()))
            .clone()
    }
    pub fn sentinel(&self) -> Arc<SentinelModule> {
        self.sentinel_module
            .get_or_init(|| SentinelModule::new(self.arc()))
            .clone()
    }
    pub fn skin(&self) -> Arc<SkinModule> {
        self.skin_module
            .get_or_init(|| SkinModule::new(self.arc()))
            .clone()
    }
    pub fn tradable_item(&self) -> Arc<TradableItemModule> {
        self.tradable_item_module
            .get_or_init(|| TradableItemModule::new(self.arc()))
            .clone()
    }
    pub fn warframe(&self) -> Arc<WarframeModule> {
        self.warframe_module
            .get_or_init(|| WarframeModule::new(self.arc()))
            .clone()
    }
    pub fn theme(&self) -> Arc<ThemeModule> {
        self.theme_module
            .get_or_init(|| ThemeModule::new(self.arc()))
            .clone()
    }
}
