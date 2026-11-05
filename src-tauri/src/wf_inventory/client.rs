use crate::{
    app::Settings,
    wf_inventory::{inv_sources::*, modules::*, WarframeRootObject},
};
use std::sync::{Arc, Mutex, OnceLock};
use utils::*;

pub struct WFInventoryState {
    source: Mutex<WFInventorySource>,
    root: Arc<Mutex<WarframeRootObject>>,
    item_module: OnceLock<Arc<ItemModule>>,
    riven_module: OnceLock<Arc<RivenModule>>,
    syndicate_module: OnceLock<Arc<SyndicateModule>>,
    mods_module: OnceLock<Arc<ModsModule>>,
}

impl WFInventoryState {
    pub fn new(settings: &Settings) -> Arc<Self> {
        let source = settings.wf_inventory.source.clone();
        info(
            "WFInventoryState:New",
            format!("Starting inventory source: {}", source),
            &LoggerOptions::default(),
        );
        let state = Arc::new(Self {
            source: Mutex::new(source),
            root: Arc::new(Mutex::new(WarframeRootObject::default())),
            item_module: OnceLock::new(),
            riven_module: OnceLock::new(),
            syndicate_module: OnceLock::new(),
            mods_module: OnceLock::new(),
        });

        // Start the source (initial load + watcher for alecaframe)
        state.source.lock().unwrap().start(&state.root);
        state.init_modules();
        state
    }

    pub fn get_root(&self) -> WarframeRootObject {
        let root = self.root.lock().unwrap().clone();
        root
    }

    pub fn update(&self) -> Result<(), Error> {
        self.source.lock().unwrap().update(&self.root)
    }

    pub fn set_source(&self, source: WFInventorySource) {
        self.source.lock().unwrap().stop();
        *self.source.lock().unwrap() = source;
        self.source.lock().unwrap().start(&self.root);
    }

    fn init_modules(self: &Arc<Self>) {
        self.item_module
            .get_or_init(|| ItemModule::new(self.clone()));
        self.riven_module
            .get_or_init(|| RivenModule::new(self.clone()));
        self.syndicate_module
            .get_or_init(|| SyndicateModule::new(self.clone()));
        self.mods_module
            .get_or_init(|| ModsModule::new(self.clone()));
    }

    // pub fn item(&self) -> Arc<ItemModule> {
    //     self.item_module
    //         .get()
    //         .expect("ItemModule not initialized")
    //         .clone()
    // }

    pub fn riven(&self) -> Arc<RivenModule> {
        self.riven_module
            .get()
            .expect("RivenModule not initialized")
            .clone()
    }
    pub fn syndicate(&self) -> Arc<SyndicateModule> {
        self.syndicate_module
            .get()
            .expect("SyndicateModule not initialized")
            .clone()
    }
    pub fn mods(&self) -> Arc<ModsModule> {
        self.mods_module
            .get()
            .expect("ModsModule not initialized")
            .clone()
    }
}
