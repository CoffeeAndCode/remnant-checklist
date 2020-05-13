use super::data;
use super::Entry;
use yew::format::Json;
use yew::services::storage::{Area, StorageService as YewStorageService};

const KEY: &str = "yew.todomvc.self";

pub struct StorageService {
    storage_service: YewStorageService,
}

impl StorageService {
    pub fn new() -> Result<Self, &'static str> {
        YewStorageService::new(Area::Local)
            .map(|storage_service| StorageService { storage_service })
            .map_err(|error| error)
    }

    pub fn restore_or_default(&self) -> Vec<Entry> {
        if let Json(Ok(restored_entries)) = self.storage_service.restore(KEY) {
            restored_entries
        } else {
            let mut entries = data::remnant_trait_entries();
            entries.append(&mut data::amulet_entries());
            entries.append(&mut data::armor_set_entries());
            entries.append(&mut data::head_armor_entries());
            entries.append(&mut data::body_armor_entries());
            entries.append(&mut data::leg_armor_entries());
            entries.append(&mut data::emote_entries());
            entries.append(&mut data::ring_entries());
            entries.append(&mut data::hand_gun_entries());
            entries.append(&mut data::long_gun_entries());
            entries.append(&mut data::melee_weapon_entries());
            entries
        }
    }

    #[allow(clippy::ptr_arg)]
    pub fn store(&mut self, value: &Vec<Entry>) {
        self.storage_service.store(KEY, Json(value))
    }
}
