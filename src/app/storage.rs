use yew::format::Text;
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

    pub fn restore<T>(&self) -> T
    where
        T: From<Text>,
    {
        self.storage_service.restore(KEY)
    }

    pub fn store<T>(&mut self, value: T)
    where
        T: Into<Text>,
    {
        self.storage_service.store(KEY, value)
    }
}
