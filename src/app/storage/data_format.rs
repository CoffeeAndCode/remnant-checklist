use crate::app::data::ItemType;
use crate::app::Entry;
use chrono::{DateTime, Utc};
use serde_derive::{Deserialize, Serialize};

const DATA_FORMAT_VERSION: usize = 1;

#[derive(Deserialize, Serialize)]
pub struct DataFormat {
    pub completed_items: Vec<Item>,
    last_saved_at: DateTime<Utc>,
    version: usize,
}

impl DataFormat {
    pub fn new(entries: &[Entry]) -> Self {
        Self {
            completed_items: entries
                .iter()
                .filter_map(|entry| {
                    if entry.completed {
                        Some(Item::from(entry))
                    } else {
                        None
                    }
                })
                .collect(),
            ..Self::default()
        }
    }
}

impl From<&Entry> for Item {
    fn from(entry: &Entry) -> Self {
        Self {
            data_type: entry.data_type,
            id: entry.id,
        }
    }
}

impl Default for DataFormat {
    fn default() -> Self {
        Self {
            completed_items: Vec::<Item>::default(),
            last_saved_at: Utc::now(),
            version: DATA_FORMAT_VERSION,
        }
    }
}

#[derive(Deserialize, Serialize)]
pub struct Item {
    pub data_type: ItemType,
    pub id: u32,
}

#[cfg(test)]
#[allow(clippy::wildcard_imports)]
mod tests {
    use super::*;
    use chrono::Duration;

    #[test]
    fn test_data_format_defaults() {
        let now = Utc::now().checked_add_signed(Duration::seconds(1)).unwrap();
        let data: DataFormat = DataFormat::default();

        assert!(data.completed_items.is_empty());
        assert_eq!(DATA_FORMAT_VERSION, data.version);
        assert!(now > data.last_saved_at);
    }

    #[test]
    fn test_data_format_version_can_differ() {
        let data = DataFormat {
            version: 42,
            ..DataFormat::default()
        };

        assert_eq!(42, data.version);
    }

    mod new {
        use super::*;

        fn build_entry(completed: bool, data_type: ItemType, id: u32) -> Entry {
            Entry {
                completed,
                data_type,
                id,
                name: "Necklace".into(),
                url: "https://example.com".into(),
                worlds: vec![],
            }
        }

        #[test]
        fn test_new_converts_completd_entries_into_items() {
            let entries = vec![
                build_entry(true, ItemType::Amulet, 4),
                build_entry(false, ItemType::LegArmor, 5),
                build_entry(true, ItemType::Ring, 6),
                build_entry(false, ItemType::HeadArmor, 7),
            ];
            let completed_items = DataFormat::new(&entries).completed_items;

            let first_item = completed_items.first().unwrap();
            assert!(matches!(first_item.data_type, ItemType::Amulet));
            assert_eq!(4, first_item.id);

            let last_item = completed_items.last().unwrap();
            assert!(matches!(last_item.data_type, ItemType::Ring));
            assert_eq!(6, last_item.id);
        }
    }
}
