use crate::app::data::DataType;
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
        DataFormat {
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
            ..Default::default()
        }
    }
}

impl From<&Entry> for Item {
    fn from(entry: &Entry) -> Self {
        Item {
            data_type: entry.data_type,
            id: entry.id,
        }
    }
}

impl Default for DataFormat {
    fn default() -> Self {
        DataFormat {
            completed_items: Default::default(),
            last_saved_at: Utc::now(),
            version: DATA_FORMAT_VERSION,
        }
    }
}

#[derive(Deserialize, Serialize)]
pub struct Item {
    pub data_type: DataType,
    pub id: u32,
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Duration;

    #[test]
    fn test_data_format_defaults() {
        let now = Utc::now().checked_add_signed(Duration::seconds(1)).unwrap();
        let data: DataFormat = Default::default();

        assert!(data.completed_items.is_empty());
        assert_eq!(DATA_FORMAT_VERSION, data.version);
        assert!(now > data.last_saved_at);
    }

    #[test]
    fn test_data_format_version_can_differ() {
        let data = DataFormat {
            version: 42,
            ..Default::default()
        };

        assert_eq!(42, data.version);
    }

    mod new {
        use super::*;

        fn build_entry(completed: bool, data_type: DataType, id: u32) -> Entry {
            Entry {
                completed,
                data_type,
                id,
                name: "Necklace".into(),
                url: "https://example.com".into(),
            }
        }

        #[test]
        fn test_new_converts_completd_entries_into_items() {
            let entries = vec![
                build_entry(true, DataType::Amulet, 4),
                build_entry(false, DataType::LegArmor, 5),
                build_entry(true, DataType::Ring, 6),
                build_entry(false, DataType::HeadArmor, 7),
            ];
            let completed_items = DataFormat::new(&entries).completed_items;

            let item = completed_items.first().unwrap();
            assert!(matches!(item.data_type, DataType::Amulet));
            assert_eq!(4, item.id);

            let item = completed_items.last().unwrap();
            assert!(matches!(item.data_type, DataType::Ring));
            assert_eq!(6, item.id);
        }
    }
}
