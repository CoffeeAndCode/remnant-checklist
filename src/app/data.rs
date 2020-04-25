use super::Entry;
use serde_derive::Deserialize;

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
            description: emote.name,
            editing: false,
        }
    }
}

impl From<Trait> for Entry {
    fn from(remnant_trait: Trait) -> Self {
        Entry {
            completed: false,
            description: remnant_trait.name,
            editing: false,
        }
    }
}

const EMOTES: &'static [u8] = include_bytes!("../data/emotes.csv");
pub fn emotes() -> Vec<Entry> {
    let mut rdr = csv::Reader::from_reader(EMOTES);
    rdr.deserialize()
        .filter_map(|t: Result<Emote, _>| t.ok())
        .map(|t| Entry::from(t))
        .collect()
}

const TRAITS: &'static [u8] = include_bytes!("../data/traits.csv");
pub fn remnant_traits() -> Vec<Entry> {
    let mut rdr = csv::Reader::from_reader(TRAITS);
    rdr.deserialize()
        .filter_map(|t: Result<Trait, _>| t.ok())
        .map(|t| Entry::from(t))
        .collect()
}

#[cfg(test)]
mod tests {
    #[test]
    fn all_emotes_found() {
        assert_eq!(15, super::emotes().len());
    }

    #[test]
    fn all_traits_found() {
        assert_eq!(32, super::remnant_traits().len());
    }
}
