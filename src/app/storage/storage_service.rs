use super::data_format::DataFormat;
use crate::app::data;
use crate::app::Entry;
use yew::format::Json;
use yew::services::storage::{Area, StorageService as YewStorageService};

const KEY: &str = "dev.coffee.remnant";

pub struct StorageService {
    storage_service: YewStorageService,
}

impl StorageService {
    pub fn new() -> Result<Self, &'static str> {
        YewStorageService::new(Area::Local)
            .map(|storage_service| StorageService { storage_service })
            .map_err(|error| error)
    }

    pub fn restore(&self) -> Vec<Entry> {
        let data = self.retrieve_stored_data();

        let mut entries = data::remnant_trait_entries(&data.completed_items);
        entries.append(&mut data::amulet_entries(&data.completed_items));
        entries.append(&mut data::armor_set_entries(&data.completed_items));
        entries.append(&mut data::head_armor_entries(&data.completed_items));
        entries.append(&mut data::body_armor_entries(&data.completed_items));
        entries.append(&mut data::leg_armor_entries(&data.completed_items));
        entries.append(&mut data::emote_entries(&data.completed_items));
        entries.append(&mut data::ring_entries(&data.completed_items));
        entries.append(&mut data::hand_gun_entries(&data.completed_items));
        entries.append(&mut data::long_gun_entries(&data.completed_items));
        entries.append(&mut data::melee_weapon_entries(&data.completed_items));
        entries.append(&mut data::mod_entries(&data.completed_items));
        entries
    }

    #[allow(clippy::ptr_arg)]
    pub fn store(&mut self, value: &Vec<Entry>) {
        self.storage_service
            .store(KEY, Json(&DataFormat::new(value)));
    }
}

impl StorageService {
    fn retrieve_stored_data(&self) -> DataFormat {
        if let Json(Ok(restored_data)) = self.storage_service.restore(KEY) {
            restored_data
        } else {
            DataFormat::default()
        }
    }
}
