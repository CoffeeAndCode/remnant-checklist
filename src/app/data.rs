use super::Entry;
use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub enum DataType {
    Emote,
    Trait,
}

impl DataType {
    fn data(&self) -> &'static [u8] {
        match self {
            DataType::Emote => include_bytes!("../data/emotes.csv"),
            DataType::Trait => include_bytes!("../data/traits.csv"),
        }
    }
}

pub trait DataDisplay {
    fn label(&self) -> &'static str;
}

impl DataDisplay for DataType {
    fn label(&self) -> &'static str {
        match self {
            DataType::Emote => "Emotes",
            DataType::Trait => "Traits",
        }
    }
}

#[derive(Debug, Deserialize)]
struct Emote {
    #[serde(rename = "Description")]
    description: String,

    #[serde(rename = "ID")]
    id: u32,

    #[serde(rename = "Location")]
    location: String,

    #[serde(rename = "Name")]
    name: String,
}

#[derive(Debug, Deserialize)]
struct Trait {
    #[serde(rename = "Description")]
    description: String,

    #[serde(rename = "ID")]
    id: u32,

    #[serde(rename = "Name")]
    name: String,
}

impl From<Emote> for Entry {
    fn from(emote: Emote) -> Self {
        Entry {
            completed: false,
            data_type: DataType::Emote,
            description: emote.name,
            editing: false,
        }
    }
}

impl From<Trait> for Entry {
    fn from(remnant_trait: Trait) -> Self {
        Entry {
            completed: false,
            data_type: DataType::Trait,
            description: remnant_trait.name,
            editing: false,
        }
    }
}

pub fn emotes() -> Vec<Entry> {
    let mut rdr = csv::Reader::from_reader(DataType::Emote.data());
    rdr.deserialize()
        .filter_map(|t: Result<Emote, _>| t.ok())
        .map(|t| Entry::from(t))
        .collect()
}

pub fn remnant_traits() -> Vec<Entry> {
    let mut rdr = csv::Reader::from_reader(DataType::Trait.data());
    rdr.deserialize()
        .filter_map(|t: Result<Trait, _>| t.ok())
        .map(|t| Entry::from(t))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn all_emotes_found() {
        assert_eq!(15, emotes().len());
    }

    #[test]
    fn all_traits_found() {
        assert_eq!(32, remnant_traits().len());
    }
}
