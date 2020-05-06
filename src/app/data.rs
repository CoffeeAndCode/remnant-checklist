use super::Entry;
use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub enum DataType {
    Emote,
    HandGun,
    MeleeWeapon,
    Trait,
}

impl DataType {
    fn data(&self) -> &'static [u8] {
        match *self {
            DataType::Emote => include_bytes!("../data/emotes.csv"),
            DataType::HandGun => include_bytes!("../data/hand_guns.csv"),
            DataType::MeleeWeapon => include_bytes!("../data/melee_weapons.csv"),
            DataType::Trait => include_bytes!("../data/traits.csv"),
        }
    }
}

pub trait DataDisplay {
    fn icon(&self) -> char;
    fn label(&self) -> &'static str;
}

impl DataDisplay for DataType {
    fn icon(&self) -> char {
        match *self {
            DataType::Emote => '☺',
            DataType::HandGun => '⚒',
            DataType::MeleeWeapon => '⚒',
            DataType::Trait => '☯',
        }
    }

    fn label(&self) -> &'static str {
        match *self {
            DataType::Emote => "Emotes",
            DataType::HandGun => "Hand Guns",
            DataType::MeleeWeapon => "Melee Weapons",
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
struct HandGun {
    #[serde(rename = "Base Damage")]
    base_damage: u32,

    #[serde(rename = "Crit Chance")]
    crit_chance: u32,

    #[serde(rename = "ID")]
    id: u32,

    #[serde(rename = "Ideal Range")]
    ideal_range: u32,

    #[serde(rename = "Magazine")]
    magazine: u32,

    #[serde(rename = "Max Ammo")]
    max_ammo: u32,

    #[serde(rename = "Max Damage")]
    max_damage: Option<u32>,

    #[serde(rename = "Name")]
    name: String,

    #[serde(rename = "RPS")]
    rps: f32,
}

#[derive(Debug, Deserialize)]
struct MeleeWeapon {
    #[serde(rename = "Base Damage")]
    base_damage: u32,

    #[serde(rename = "ID")]
    id: u32,

    #[serde(rename = "Max Damage")]
    max_damage: Option<u32>,

    #[serde(rename = "Name")]
    name: String,

    #[serde(rename = "Weapon Mod")]
    weapon_mod: Option<String>,
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
            description: format!("{} {}", emote.name, DataType::Emote.icon()),
            editing: false,
        }
    }
}

impl From<HandGun> for Entry {
    fn from(hand_gun: HandGun) -> Self {
        Entry {
            completed: false,
            data_type: DataType::HandGun,
            description: format!("{} {}", hand_gun.name, DataType::HandGun.icon()),
            editing: false,
        }
    }
}

impl From<MeleeWeapon> for Entry {
    fn from(melee_weapon: MeleeWeapon) -> Self {
        Entry {
            completed: false,
            data_type: DataType::MeleeWeapon,
            description: format!("{} {}", melee_weapon.name, DataType::MeleeWeapon.icon()),
            editing: false,
        }
    }
}

impl From<Trait> for Entry {
    fn from(remnant_trait: Trait) -> Self {
        Entry {
            completed: false,
            data_type: DataType::Trait,
            description: format!("{} {}", remnant_trait.name, DataType::Trait.icon()),
            editing: false,
        }
    }
}

pub fn emotes() -> Vec<Entry> {
    let mut rdr = csv::Reader::from_reader(DataType::Emote.data());
    rdr.deserialize()
        .filter_map(|t: Result<Emote, _>| t.ok())
        .map(Entry::from)
        .collect()
}

pub fn hand_guns() -> Vec<Entry> {
    let mut rdr = csv::Reader::from_reader(DataType::HandGun.data());
    rdr.deserialize()
        .filter_map(|t: Result<HandGun, _>| t.ok())
        .map(Entry::from)
        .collect()
}

pub fn melee_weapons() -> Vec<Entry> {
    let mut rdr = csv::Reader::from_reader(DataType::MeleeWeapon.data());
    rdr.deserialize()
        .filter_map(|t: Result<MeleeWeapon, _>| t.ok())
        .map(Entry::from)
        .collect()
}

pub fn remnant_traits() -> Vec<Entry> {
    let mut rdr = csv::Reader::from_reader(DataType::Trait.data());
    rdr.deserialize()
        .filter_map(|t: Result<Trait, _>| t.ok())
        .map(Entry::from)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn all_hand_guns_found() {
        assert_eq!(9, hand_guns().len());
    }

    #[test]
    fn all_emotes_found() {
        assert_eq!(15, emotes().len());
    }

    // #[test]
    // fn all_long_guns_found() {
    //     assert_eq!(15, long_guns().len());
    // }

    #[test]
    fn all_melee_weapons_found() {
        assert_eq!(17, melee_weapons().len());
    }

    #[test]
    fn all_traits_found() {
        assert_eq!(40, remnant_traits().len());
    }
}
