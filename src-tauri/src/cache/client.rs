use std::{
    fs::File,
    io::{Read, Write},
    path::PathBuf,
    sync::{Arc, Mutex, RwLock},
};

use eyre::eyre;



use crate::{
    helper, logger, settings::SettingsState, utils::modules::error::AppError
};

use super::modules::{
    arcane::ArcaneModule, arch_gun::ArchGunModule, arch_melee::ArchMeleeModule, archwing::ArchwingModule, fish::FishModule, item_price::ItemPriceModule, melee::MeleeModule, misc::MiscModule, mods::ModModule, parts::PartModule, pet::PetModule, primary::PrimaryModule, relics::RelicsModule, resource::ResourceModule, riven::RivenModule, secondary::SecondaryModule, sentinel::SentinelModule, skin::SkinModule, tradable_items::TradableItemModule, warframe::WarframeModule
};

#[derive(Clone, Debug)]
pub struct CacheClient {
    pub log_file: PathBuf,
    pub qf: Arc<Mutex<crate::qf_client::client::QFClient>>,
    pub settings: Arc<Mutex<SettingsState>>,
    item_price_module: Arc<RwLock<Option<ItemPriceModule>>>,
    relics_module: Arc<RwLock<Option<RelicsModule>>>,
    riven_module: Arc<RwLock<Option<RivenModule>>>,
    arcane_module: Arc<RwLock<Option<ArcaneModule>>>,
    warframe_module: Arc<RwLock<Option<WarframeModule>>>,
    arch_gun_module: Arc<RwLock<Option<ArchGunModule>>>,
    arch_melee_module: Arc<RwLock<Option<ArchMeleeModule>>>,
    archwing_module: Arc<RwLock<Option<ArchwingModule>>>,
    melee_module: Arc<RwLock<Option<MeleeModule>>>,
    mods_module: Arc<RwLock<Option<ModModule>>>,
    primary_module: Arc<RwLock<Option<PrimaryModule>>>,
    secondary_module: Arc<RwLock<Option<SecondaryModule>>>,
    sentinel_module: Arc<RwLock<Option<SentinelModule>>>,
    tradable_items_module: Arc<RwLock<Option<TradableItemModule>>>,
    skin_module: Arc<RwLock<Option<SkinModule>>>,
    misc_module: Arc<RwLock<Option<MiscModule>>>,
    pet_module: Arc<RwLock<Option<PetModule>>>,
    resource_module: Arc<RwLock<Option<ResourceModule>>>,
    part_module: Arc<RwLock<Option<PartModule>>>,
    fish_module: Arc<RwLock<Option<FishModule>>>,
    pub component: String,
    pub cache_path: PathBuf,
    md5_file: String,
}

impl CacheClient {
    pub fn new(
        qf: Arc<Mutex<crate::qf_client::client::QFClient>>,
        settings: Arc<Mutex<SettingsState>>,
    ) -> Self {
        CacheClient {
            log_file: PathBuf::from("cache"),
            qf,
            settings,
            component: "Cache".to_string(),
            md5_file: "cache_id.txt".to_string(),
            item_price_module: Arc::new(RwLock::new(None)),
            riven_module: Arc::new(RwLock::new(None)),
            relics_module: Arc::new(RwLock::new(None)),
            arcane_module: Arc::new(RwLock::new(None)),
            warframe_module: Arc::new(RwLock::new(None)),
            arch_gun_module: Arc::new(RwLock::new(None)),
            arch_melee_module: Arc::new(RwLock::new(None)),
            archwing_module: Arc::new(RwLock::new(None)),
            melee_module: Arc::new(RwLock::new(None)),
            mods_module: Arc::new(RwLock::new(None)),
            primary_module: Arc::new(RwLock::new(None)),
            secondary_module: Arc::new(RwLock::new(None)),
            sentinel_module: Arc::new(RwLock::new(None)),
            tradable_items_module: Arc::new(RwLock::new(None)),
            skin_module: Arc::new(RwLock::new(None)),
            misc_module: Arc::new(RwLock::new(None)),
            pet_module: Arc::new(RwLock::new(None)),
            resource_module: Arc::new(RwLock::new(None)),
            part_module: Arc::new(RwLock::new(None)),
            fish_module: Arc::new(RwLock::new(None)),
            cache_path: helper::get_app_storage_path().join("cache"),
        }
    }

    pub fn update_current_cache_id(&self, cache_id: String) -> Result<(), AppError> {
        let cache_path = self.cache_path.join(self.md5_file.clone());
        let mut file = File::create(cache_path)
            .map_err(|e| AppError::new(&self.component, eyre!(e.to_string())))?;

        file.write_all(cache_id.as_bytes())
            .map_err(|e| AppError::new(&self.component, eyre!(e.to_string())))?;

        Ok(())
    }

    fn get_current_cache_id(&self) -> Result<String, AppError> {
        let cache_path = self.cache_path.join(self.md5_file.clone());
        if !cache_path.exists() {

            return Ok("N/A".to_string());
        }
        let mut file = File::open(cache_path)
            .map_err(|e| AppError::new(&self.component, eyre!(e.to_string())))?;

        let mut content = String::new();
        file.read_to_string(&mut content)
            .map_err(|e| AppError::new(&self.component, eyre!(e.to_string())))?;
        Ok(content)
    }

    pub async fn download_cache_data(&self) -> Result<(), AppError> {
        let qf = self.qf.lock()?.clone();
        let zip_data = qf.cache().get_zip().await?;

        let reader = std::io::Cursor::new(zip_data);
        let mut archive = zip::ZipArchive::new(reader)
            .map_err(|e| AppError::new(&self.component, eyre!(e.to_string())))?;

        let extract_to = helper::get_app_storage_path().join(self.cache_path.clone());

        for i in 0..archive.len() {
            let mut file = archive
                .by_index(i)
                .map_err(|e| AppError::new(&self.component, eyre!(e.to_string())))?;
            let output_path = extract_to.join(file.mangled_name());

            if file.is_dir() {
                std::fs::create_dir_all(&output_path)
                    .map_err(|e| AppError::new(&self.component, eyre!(e.to_string())))?;
            } else {
                if let Some(parent) = output_path.parent() {
                    if !parent.exists() {
                        std::fs::create_dir_all(parent)
                            .map_err(|e| AppError::new(&self.component, eyre!(e.to_string())))?;
                    }
                }

                let mut output_file = File::create(&output_path)
                    .map_err(|e| AppError::new(&self.component, eyre!(e.to_string())))?;

                std::io::copy(&mut file, &mut output_file)
                    .map_err(|e| AppError::new(&self.component, eyre!(e.to_string())))?;
            }
        }
        logger::info_con(&self.component, "Cache data downloaded and extracted");
        Ok(())
    }

    pub async fn load(&self) -> Result<(), AppError> {
        let qf = self.qf.lock()?.clone();
        let settings = self.settings.lock()?.clone();
        if !settings.dev_mode {
            let current_cache_id = self.get_current_cache_id()?;
            logger::info_con(
                &self.component,
                format!("Current cache id: {}", current_cache_id).as_str(),
            );
            let remote_cache_id = match qf.cache().get_cache_id().await {
                Ok(id) => id,
                Err(e) => {
                    logger::error_con(
                        &self.component,
                        format!(
                            "There was an error downloading the cache from the server: {:?}",
                            e
                        )
                        .as_str(),
                    );
                    logger::info_con(&self.component, "Using the current cache data");
                    current_cache_id.clone()
                }
            };
            logger::info_con(
                &self.component,
                format!("Remote cache id: {}", remote_cache_id).as_str(),
            );
            if current_cache_id != remote_cache_id {
                logger::info_con(
                    &self.component,
                    "Cache id mismatch, downloading new cache data",
                );
                self.download_cache_data().await?;
                self.update_current_cache_id(remote_cache_id)?;
            }
        }else {
            logger::warning_con(&self.component, "Dev Mode is enabled, skipping cache download using current cache data");
        }

        self.arcane().load()?;
        logger::info_con(&self.component, "Arcane data loaded");
        self.warframe().load()?;
        logger::info_con(&self.component, "Warframe data loaded");
        self.arch_gun().load()?;
        logger::info_con(&self.component, "ArchGun data loaded");
        self.arch_melee().load()?;
        logger::info_con(&self.component, "ArchMelee data loaded");
        self.archwing().load()?;
        logger::info_con(&self.component, "Archwing data loaded");
        self.melee().load()?;
        logger::info_con(&self.component, "Melee data loaded");
        self.mods().load()?;
        logger::info_con(&self.component, "Mods data loaded");
        self.primary().load()?;
        logger::info_con(&self.component, "Primary data loaded");
        self.secondary().load()?;
        logger::info_con(&self.component, "Secondary data loaded");
        self.sentinel().load()?;
        logger::info_con(&self.component, "Sentinel data loaded");
        self.tradable_items().load()?;
        logger::info_con(&self.component, "Tradable items data loaded");
        self.skin().load()?;
        logger::info_con(&self.component, "Skin data loaded");
        self.misc().load()?;
        logger::info_con(&self.component, "Misc data loaded");
        self.pet().load()?;
        logger::info_con(&self.component, "Pet data loaded");
        self.fish().load()?;
        logger::info_con(&self.component, "Fish data loaded");
        self.resource().load()?;
        logger::info_con(&self.component, "Resource data loaded");
        self.riven().load()?;
        logger::info_con(&self.component, "Riven data loaded");
        self.parts().load()?;
        logger::info_con(&self.component, "Parts data loaded");
        self.item_price().load().await?;
        logger::info_con(&self.component, "Item price data loaded");
        self.relics().load()?;
        logger::info_con(&self.component, "Relics data loaded");
        return Ok(());
    }

    pub fn item_price(&self) -> ItemPriceModule {
        // Lazily initialize ItemModule if not already initialized
        if self.item_price_module.read().unwrap().is_none() {
            *self.item_price_module.write().unwrap() =
                Some(ItemPriceModule::new(self.clone()).clone());
        }

        // Unwrapping is safe here because we ensured the order_module is initialized
        self.item_price_module
            .read()
            .unwrap()
            .as_ref()
            .unwrap()
            .clone()
    }
    pub fn update_item_price_module(&self, module: ItemPriceModule) {
        // Update the stored ItemModule
        *self.item_price_module.write().unwrap() = Some(module);
    }

    pub fn riven(&self) -> RivenModule {
        // Lazily initialize ItemModule if not already initialized
        if self.riven_module.read().unwrap().is_none() {
            *self.riven_module.write().unwrap() = Some(RivenModule::new(self.clone()).clone());
        }

        // Unwrapping is safe here because we ensured the order_module is initialized
        self.riven_module.read().unwrap().as_ref().unwrap().clone()
    }
    pub fn update_riven_module(&self, module: RivenModule) {
        // Update the stored ItemModule
        *self.riven_module.write().unwrap() = Some(module);
    }

    pub fn relics(&self) -> RelicsModule {
        // Lazily initialize RelicsModule if not already initialized
        if self.relics_module.read().unwrap().is_none() {
            *self.relics_module.write().unwrap() = Some(RelicsModule::new(self.clone()).clone());
        }

        // Unwrapping is safe here because we ensured the relics_module is initialized
        self.relics_module.read().unwrap().as_ref().unwrap().clone()
    }

    pub fn update_relics_module(&self, module: RelicsModule) {
        // Update the stored RelicsModule
        *self.relics_module.write().unwrap() = Some(module);
    }

    pub fn arcane(&self) -> ArcaneModule {
        // Lazily initialize ArcaneModule if not already initialized
        if self.arcane_module.read().unwrap().is_none() {
            *self.arcane_module.write().unwrap() = Some(ArcaneModule::new(self.clone()).clone());
        }

        // Unwrapping is safe here because we ensured the arcane_module is initialized
        self.arcane_module.read().unwrap().as_ref().unwrap().clone()
    }
    pub fn update_arcane_module(&self, module: ArcaneModule) {
        // Update the stored ArcaneModule
        *self.arcane_module.write().unwrap() = Some(module);
    }

    pub fn arch_gun(&self) -> ArchGunModule {
        // Lazily initialize ArchGunModule if not already initialized
        if self.arch_gun_module.read().unwrap().is_none() {
            *self.arch_gun_module.write().unwrap() = Some(ArchGunModule::new(self.clone()).clone());
        }

        // Unwrapping is safe here because we ensured the arch_gun_module is initialized
        self.arch_gun_module
            .read()
            .unwrap()
            .as_ref()
            .unwrap()
            .clone()
    }
    pub fn update_arch_gun_module(&self, module: ArchGunModule) {
        // Update the stored ArchGunModule
        *self.arch_gun_module.write().unwrap() = Some(module);
    }

    pub fn arch_melee(&self) -> ArchMeleeModule {
        // Lazily initialize ArchMeleeModule if not already initialized
        if self.arch_melee_module.read().unwrap().is_none() {
            *self.arch_melee_module.write().unwrap() =
                Some(ArchMeleeModule::new(self.clone()).clone());
        }

        // Unwrapping is safe here because we ensured the arch_melee_module is initialized
        self.arch_melee_module
            .read()
            .unwrap()
            .as_ref()
            .unwrap()
            .clone()
    }
    pub fn update_arch_melee_module(&self, module: ArchMeleeModule) {
        // Update the stored ArchMeleeModule
        *self.arch_melee_module.write().unwrap() = Some(module);
    }

    pub fn archwing(&self) -> ArchwingModule {
        // Lazily initialize ArchwingModule if not already initialized
        if self.archwing_module.read().unwrap().is_none() {
            *self.archwing_module.write().unwrap() =
                Some(ArchwingModule::new(self.clone()).clone());
        }

        // Unwrapping is safe here because we ensured the archwing_module is initialized
        self.archwing_module
            .read()
            .unwrap()
            .as_ref()
            .unwrap()
            .clone()
    }
    pub fn update_archwing_module(&self, module: ArchwingModule) {
        // Update the stored ArchwingModule
        *self.archwing_module.write().unwrap() = Some(module);
    }

    pub fn melee(&self) -> MeleeModule {
        // Lazily initialize MeleeModule if not already initialized
        if self.melee_module.read().unwrap().is_none() {
            *self.melee_module.write().unwrap() = Some(MeleeModule::new(self.clone()).clone());
        }

        // Unwrapping is safe here because we ensured the melee_module is initialized
        self.melee_module.read().unwrap().as_ref().unwrap().clone()
    }
    pub fn update_melee_module(&self, module: MeleeModule) {
        // Update the stored MeleeModule
        *self.melee_module.write().unwrap() = Some(module);
    }

    pub fn mods(&self) -> ModModule {
        // Lazily initialize ModModule if not already initialized
        if self.mods_module.read().unwrap().is_none() {
            *self.mods_module.write().unwrap() = Some(ModModule::new(self.clone()).clone());
        }

        // Unwrapping is safe here because we ensured the mods_module is initialized
        self.mods_module.read().unwrap().as_ref().unwrap().clone()
    }
    pub fn update_mods_module(&self, module: ModModule) {
        // Update the stored ModModule
        *self.mods_module.write().unwrap() = Some(module);
    }

    pub fn primary(&self) -> PrimaryModule {
        // Lazily initialize PrimaryModule if not already initialized
        if self.primary_module.read().unwrap().is_none() {
            *self.primary_module.write().unwrap() = Some(PrimaryModule::new(self.clone()).clone());
        }

        // Unwrapping is safe here because we ensured the primary_module is initialized
        self.primary_module
            .read()
            .unwrap()
            .as_ref()
            .unwrap()
            .clone()
    }
    pub fn update_primary_module(&self, module: PrimaryModule) {
        // Update the stored PrimaryModule
        *self.primary_module.write().unwrap() = Some(module);
    }

    pub fn secondary(&self) -> SecondaryModule {
        // Lazily initialize SecondaryModule if not already initialized
        if self.secondary_module.read().unwrap().is_none() {
            *self.secondary_module.write().unwrap() =
                Some(SecondaryModule::new(self.clone()).clone());
        }

        // Unwrapping is safe here because we ensured the secondary_module is initialized
        self.secondary_module
            .read()
            .unwrap()
            .as_ref()
            .unwrap()
            .clone()
    }
    pub fn update_secondary_module(&self, module: SecondaryModule) {
        // Update the stored SecondaryModule
        *self.secondary_module.write().unwrap() = Some(module);
    }

    pub fn sentinel(&self) -> SentinelModule {
        // Lazily initialize SentinelModule if not already initialized
        if self.sentinel_module.read().unwrap().is_none() {
            *self.sentinel_module.write().unwrap() =
                Some(SentinelModule::new(self.clone()).clone());
        }

        // Unwrapping is safe here because we ensured the sentinel_module is initialized
        self.sentinel_module
            .read()
            .unwrap()
            .as_ref()
            .unwrap()
            .clone()
    }
    pub fn update_sentinel_module(&self, module: SentinelModule) {
        // Update the stored SentinelModule
        *self.sentinel_module.write().unwrap() = Some(module);
    }

    pub fn warframe(&self) -> WarframeModule {
        // Lazily initialize ArcaneModule if not already initialized
        if self.warframe_module.read().unwrap().is_none() {
            *self.warframe_module.write().unwrap() =
                Some(WarframeModule::new(self.clone()).clone());
        }

        // Unwrapping is safe here because we ensured the warframe_module is initialized
        self.warframe_module
            .read()
            .unwrap()
            .as_ref()
            .unwrap()
            .clone()
    }
    pub fn update_warframe_module(&self, module: WarframeModule) {
        // Update the stored WarframeModule
        *self.warframe_module.write().unwrap() = Some(module);
    }

    pub fn tradable_items(&self) -> TradableItemModule {
        // Lazily initialize ArcaneModule if not already initialized
        if self.tradable_items_module.read().unwrap().is_none() {
            *self.tradable_items_module.write().unwrap() =
                Some(TradableItemModule::new(self.clone()).clone());
        }

        // Unwrapping is safe here because we ensured the tradable_items_module is initialized
        self.tradable_items_module
            .read()
            .unwrap()
            .as_ref()
            .unwrap()
            .clone()
    }
    pub fn update_tradable_items_module(&self, module: TradableItemModule) {
        // Update the stored Warframe
        *self.tradable_items_module.write().unwrap() = Some(module);
    }

    pub fn resource(&self) -> ResourceModule {
        // Lazily initialize ResourceModule if not already initialized
        if self.resource_module.read().unwrap().is_none() {
            *self.resource_module.write().unwrap() =
                Some(ResourceModule::new(self.clone()).clone());
        }

        // Unwrapping is safe here because we ensured the order_module is initialized
        self.resource_module
            .read()
            .unwrap()
            .as_ref()
            .unwrap()
            .clone()
    }
    pub fn update_resource_module(&self, module: ResourceModule) {
        // Update the stored ResourceModule
        *self.resource_module.write().unwrap() = Some(module);
    }

    pub fn misc(&self) -> MiscModule {
        // Lazily initialize MiscModule if not already initialized
        if self.misc_module.read().unwrap().is_none() {
            *self.misc_module.write().unwrap() = Some(MiscModule::new(self.clone()).clone());
        }

        // Unwrapping is safe here because we ensured the order_module is initialized
        self.misc_module.read().unwrap().as_ref().unwrap().clone()
    }
    pub fn update_misc_module(&self, module: MiscModule) {
        // Update the stored MiscModule
        *self.misc_module.write().unwrap() = Some(module);
    }

    pub fn pet(&self) -> PetModule {
        // Lazily initialize PetModule if not already initialized
        if self.pet_module.read().unwrap().is_none() {
            *self.pet_module.write().unwrap() = Some(PetModule::new(self.clone()).clone());
        }

        // Unwrapping is safe here because we ensured the order_module is initialized
        self.pet_module.read().unwrap().as_ref().unwrap().clone()
    }
    pub fn update_pet_module(&self, module: PetModule) {
        // Update the stored PetModule
        *self.pet_module.write().unwrap() = Some(module);
    }

    pub fn fish(&self) -> FishModule {
        // Lazily initialize FishModule if not already initialized
        if self.fish_module.read().unwrap().is_none() {
            *self.fish_module.write().unwrap() = Some(FishModule::new(self.clone()).clone());
        }

        // Unwrapping is safe here because we ensured the order_module is initialized
        self.fish_module.read().unwrap().as_ref().unwrap().clone()
    }
    pub fn update_fish_module(&self, module: FishModule) {
        // Update the stored FishModule
        *self.fish_module.write().unwrap() = Some(module);
    }

    pub fn skin(&self) -> SkinModule {
        // Lazily initialize SkinModule if not already initialized
        if self.skin_module.read().unwrap().is_none() {
            *self.skin_module.write().unwrap() = Some(SkinModule::new(self.clone()).clone());
        }

        // Unwrapping is safe here because we ensured the order_module is initialized
        self.skin_module.read().unwrap().as_ref().unwrap().clone()
    }
    pub fn update_skin_module(&self, module: SkinModule) {
        // Update the stored SkinModule
        *self.skin_module.write().unwrap() = Some(module);
    }

    pub fn parts(&self) -> PartModule {
        // Lazily initialize PartModule if not already initialized
        if self.part_module.read().unwrap().is_none() {
            *self.part_module.write().unwrap() = Some(PartModule::new(self.clone()).clone());
        }

        // Unwrapping is safe here because we ensured the order_module is initialized
        self.part_module.read().unwrap().as_ref().unwrap().clone()
    }
    pub fn update_part_module(&self, module: PartModule) {
        // Update the stored PartModule
        *self.part_module.write().unwrap() = Some(module);
    }

    pub fn get_path(&self, path: &str) -> PathBuf {
        let path = self.cache_path.join(path);
        if !path.exists() {
            std::fs::create_dir_all(&path).expect("Failed to create cache directory");
        }
        path
    }

    pub fn read_text_from_file(&self, path: &PathBuf) -> Result<String, AppError> {
        let mut file = File::open(self.cache_path.join(path)).map_err(|e| {
            AppError::new(
                &self.component,
                eyre!(format!(
                    "Failed to open file: {}, error: {}",
                    path.to_str().unwrap(),
                    e.to_string()
                )),
            )
        })?;
        let mut content = String::new();
        file.read_to_string(&mut content).map_err(|e| {
            AppError::new(
                &self.component,
                eyre!(format!(
                    "Failed to read file: {}, error: {}",
                    path.to_str().unwrap(),
                    e.to_string()
                )),
            )
        })?;

        Ok(content)
    }
}
