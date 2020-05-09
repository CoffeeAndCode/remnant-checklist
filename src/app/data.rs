use super::Entry;
use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub enum DataType {
    Amulet,
    ArmorSet,
    BodyArmor,
    Emote,
    HandGun,
    HeadArmor,
    LegArmor,
    LongGun,
    MeleeWeapon,
    Ring,
    Trait,
}

impl DataType {
    fn data(&self) -> &'static [u8] {
        match *self {
            DataType::Amulet => include_bytes!("../data/amulets.csv"),
            DataType::ArmorSet => include_bytes!("../data/armor_sets.csv"),
            DataType::BodyArmor => include_bytes!("../data/body_armor.csv"),
            DataType::Emote => include_bytes!("../data/emotes.csv"),
            DataType::HeadArmor => include_bytes!("../data/head_armor.csv"),
            DataType::HandGun => include_bytes!("../data/hand_guns.csv"),
            DataType::LegArmor => include_bytes!("../data/leg_armor.csv"),
            DataType::LongGun => include_bytes!("../data/long_guns.csv"),
            DataType::MeleeWeapon => include_bytes!("../data/melee_weapons.csv"),
            DataType::Ring => include_bytes!("../data/rings.csv"),
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
            DataType::Amulet => '💎',
            DataType::ArmorSet => '👘',
            DataType::BodyArmor => '🧥',
            DataType::Emote => '☺',
            DataType::HandGun => '⚒',
            DataType::HeadArmor => '💂',
            DataType::LegArmor => '🧦',
            DataType::LongGun => '⚒',
            DataType::MeleeWeapon => '⚒',
            DataType::Ring => '💫',
            DataType::Trait => '☯',
        }
    }

    fn label(&self) -> &'static str {
        match *self {
            DataType::Amulet => "Amulets",
            DataType::ArmorSet => "Armor Sets",
            DataType::BodyArmor => "Body Armor",
            DataType::Emote => "Emotes",
            DataType::HandGun => "Hand Guns",
            DataType::HeadArmor => "Head Armor",
            DataType::LegArmor => "Leg Armor",
            DataType::LongGun => "Long Guns",
            DataType::MeleeWeapon => "Melee Weapons",
            DataType::Ring => "Rings",
            DataType::Trait => "Traits",
        }
    }
}

#[derive(Debug, Deserialize)]
struct Amulet {
    #[serde(rename = "Description")]
    description: Option<String>,

    #[serde(rename = "ID")]
    id: u32,

    #[serde(rename = "Location & Crafting")]
    location: Option<String>,

    #[serde(rename = "Name")]
    name: String,
}

#[derive(Debug, Deserialize)]
struct ArmorSet {
    #[serde(rename = "ID")]
    id: u32,

    #[serde(rename = "Name")]
    name: String,

    #[serde(rename = "Set Bonus")]
    set_bonus: String,
}

#[derive(Debug, Deserialize)]
struct BodyArmor {
    #[serde(rename = "Armor")]
    armor: f32,

    #[serde(rename = "Armor Skill")]
    armor_skill: String,

    #[serde(rename = "Bleed")]
    bleed: f32,

    #[serde(rename = "Corrosive")]
    corrosive: f32,

    #[serde(rename = "Fire")]
    fire: f32,

    #[serde(rename = "Radiation")]
    radiation: f32,

    #[serde(rename = "Rot")]
    rot: f32,

    #[serde(rename = "Shock")]
    shock: f32,

    #[serde(rename = "Weight")]
    weight: f32,

    #[serde(rename = "ID")]
    id: u32,

    #[serde(rename = "Name")]
    name: String,
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
struct HeadArmor {
    #[serde(rename = "Armor")]
    armor: f32,

    #[serde(rename = "Armor Skill")]
    armor_skill: String,

    #[serde(rename = "Bleed")]
    bleed: f32,

    #[serde(rename = "Corrosive")]
    corrosive: f32,

    #[serde(rename = "Fire")]
    fire: f32,

    #[serde(rename = "Radiation")]
    radiation: f32,

    #[serde(rename = "Rot")]
    rot: f32,

    #[serde(rename = "Shock")]
    shock: f32,

    #[serde(rename = "Weight")]
    weight: f32,

    #[serde(rename = "ID")]
    id: u32,

    #[serde(rename = "Name")]
    name: String,
}

#[derive(Debug, Deserialize)]
struct LegArmor {
    #[serde(rename = "Armor")]
    armor: f32,

    #[serde(rename = "Armor Skill")]
    armor_skill: String,

    #[serde(rename = "Bleed")]
    bleed: f32,

    #[serde(rename = "Corrosive")]
    corrosive: f32,

    #[serde(rename = "Fire")]
    fire: f32,

    #[serde(rename = "Radiation")]
    radiation: f32,

    #[serde(rename = "Rot")]
    rot: f32,

    #[serde(rename = "Shock")]
    shock: f32,

    #[serde(rename = "Weight")]
    weight: f32,

    #[serde(rename = "ID")]
    id: u32,

    #[serde(rename = "Name")]
    name: String,
}

#[derive(Debug, Deserialize)]
struct LongGun {
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
struct Ring {
    #[serde(rename = "Description")]
    description: String,

    #[serde(rename = "ID")]
    id: u32,

    #[serde(rename = "Name")]
    name: String,

    #[serde(rename = "Location")]
    location: String,
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

impl From<Amulet> for Entry {
    fn from(amulet: Amulet) -> Self {
        Entry {
            completed: false,
            data_type: DataType::Amulet,
            description: format!("{} {}", amulet.name, DataType::Amulet.icon()),
        }
    }
}

impl From<ArmorSet> for Entry {
    fn from(armor_set: ArmorSet) -> Self {
        Entry {
            completed: false,
            data_type: DataType::ArmorSet,
            description: format!("{} {}", armor_set.name, DataType::ArmorSet.icon()),
        }
    }
}

impl From<BodyArmor> for Entry {
    fn from(body_armor: BodyArmor) -> Self {
        Entry {
            completed: false,
            data_type: DataType::BodyArmor,
            description: format!("{} {}", body_armor.name, DataType::BodyArmor.icon()),
        }
    }
}

impl From<Emote> for Entry {
    fn from(emote: Emote) -> Self {
        Entry {
            completed: false,
            data_type: DataType::Emote,
            description: format!("{} {}", emote.name, DataType::Emote.icon()),
        }
    }
}

impl From<HandGun> for Entry {
    fn from(hand_gun: HandGun) -> Self {
        Entry {
            completed: false,
            data_type: DataType::HandGun,
            description: format!("{} {}", hand_gun.name, DataType::HandGun.icon()),
        }
    }
}

impl From<HeadArmor> for Entry {
    fn from(head_armor: HeadArmor) -> Self {
        Entry {
            completed: false,
            data_type: DataType::HeadArmor,
            description: format!("{} {}", head_armor.name, DataType::HeadArmor.icon()),
        }
    }
}

impl From<LegArmor> for Entry {
    fn from(leg_armor: LegArmor) -> Self {
        Entry {
            completed: false,
            data_type: DataType::LegArmor,
            description: format!("{} {}", leg_armor.name, DataType::LegArmor.icon()),
        }
    }
}

impl From<LongGun> for Entry {
    fn from(long_gun: LongGun) -> Self {
        Entry {
            completed: false,
            data_type: DataType::LongGun,
            description: format!("{} {}", long_gun.name, DataType::LongGun.icon()),
        }
    }
}

impl From<MeleeWeapon> for Entry {
    fn from(melee_weapon: MeleeWeapon) -> Self {
        Entry {
            completed: false,
            data_type: DataType::MeleeWeapon,
            description: format!("{} {}", melee_weapon.name, DataType::MeleeWeapon.icon()),
        }
    }
}

impl From<Ring> for Entry {
    fn from(ring: Ring) -> Self {
        Entry {
            completed: false,
            data_type: DataType::MeleeWeapon,
            description: format!("{} {}", ring.name, DataType::Ring.icon()),
        }
    }
}

impl From<Trait> for Entry {
    fn from(remnant_trait: Trait) -> Self {
        Entry {
            completed: false,
            data_type: DataType::Trait,
            description: format!("{} {}", remnant_trait.name, DataType::Trait.icon()),
        }
    }
}

fn amulets() -> Vec<Amulet> {
    let mut rdr = csv::Reader::from_reader(DataType::Amulet.data());
    rdr.deserialize()
        .filter_map(|t: Result<Amulet, _>| t.ok())
        .collect()
}

pub fn amulet_entries() -> Vec<Entry> {
    amulets().into_iter().map(Entry::from).collect()
}

fn armor_sets() -> Vec<ArmorSet> {
    let mut rdr = csv::Reader::from_reader(DataType::ArmorSet.data());
    rdr.deserialize()
        .filter_map(|t: Result<ArmorSet, _>| t.ok())
        .collect()
}

pub fn armor_set_entries() -> Vec<Entry> {
    armor_sets().into_iter().map(Entry::from).collect()
}

fn body_armor() -> Vec<BodyArmor> {
    let mut rdr = csv::Reader::from_reader(DataType::BodyArmor.data());
    rdr.deserialize()
        .filter_map(|t: Result<BodyArmor, _>| t.ok())
        .collect()
}

pub fn body_armor_entries() -> Vec<Entry> {
    body_armor().into_iter().map(Entry::from).collect()
}

fn emotes() -> Vec<Emote> {
    let mut rdr = csv::Reader::from_reader(DataType::Emote.data());
    rdr.deserialize()
        .filter_map(|t: Result<Emote, _>| t.ok())
        .collect()
}

pub fn emote_entries() -> Vec<Entry> {
    emotes().into_iter().map(Entry::from).collect()
}

fn hand_guns() -> Vec<HandGun> {
    let mut rdr = csv::Reader::from_reader(DataType::HandGun.data());
    rdr.deserialize()
        .filter_map(|t: Result<HandGun, _>| t.ok())
        .collect()
}

pub fn hand_gun_entries() -> Vec<Entry> {
    hand_guns().into_iter().map(Entry::from).collect()
}

fn head_armor() -> Vec<HeadArmor> {
    let mut rdr = csv::Reader::from_reader(DataType::HeadArmor.data());
    rdr.deserialize()
        .filter_map(|t: Result<HeadArmor, _>| t.ok())
        .collect()
}

pub fn head_armor_entries() -> Vec<Entry> {
    head_armor().into_iter().map(Entry::from).collect()
}

fn leg_armor() -> Vec<LegArmor> {
    let mut rdr = csv::Reader::from_reader(DataType::LegArmor.data());
    rdr.deserialize()
        .filter_map(|t: Result<LegArmor, _>| t.ok())
        .collect()
}

pub fn leg_armor_entries() -> Vec<Entry> {
    leg_armor().into_iter().map(Entry::from).collect()
}

fn long_guns() -> Vec<LongGun> {
    let mut rdr = csv::Reader::from_reader(DataType::LongGun.data());
    rdr.deserialize()
        .filter_map(|t: Result<LongGun, _>| t.ok())
        .collect()
}

pub fn long_gun_entries() -> Vec<Entry> {
    long_guns().into_iter().map(Entry::from).collect()
}

fn melee_weapons() -> Vec<MeleeWeapon> {
    let mut rdr = csv::Reader::from_reader(DataType::MeleeWeapon.data());
    rdr.deserialize()
        .filter_map(|t: Result<MeleeWeapon, _>| t.ok())
        .collect()
}

pub fn melee_weapon_entries() -> Vec<Entry> {
    melee_weapons().into_iter().map(Entry::from).collect()
}

fn remnant_traits() -> Vec<Trait> {
    let mut rdr = csv::Reader::from_reader(DataType::Trait.data());
    rdr.deserialize()
        .filter_map(|t: Result<Trait, _>| t.ok())
        .collect()
}

pub fn remnant_trait_entries() -> Vec<Entry> {
    remnant_traits().into_iter().map(Entry::from).collect()
}

fn rings() -> Vec<Ring> {
    let mut rdr = csv::Reader::from_reader(DataType::Ring.data());
    rdr.deserialize()
        .filter_map(|t: Result<Ring, _>| t.ok())
        .collect()
}

pub fn ring_entries() -> Vec<Entry> {
    rings().into_iter().map(Entry::from).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const NUMBER_OF_AMULETS: usize = 22;
    const NUMBER_OF_ARMOR_SETS: usize = 17;
    const NUMBER_OF_BODY_ARMOR: usize = 16;
    const NUMBER_OF_HAND_GUNS: usize = 9;
    const NUMBER_OF_HEAD_ARMOR: usize = 18;
    const NUMBER_OF_EMOTES: usize = 15;
    const NUMBER_OF_LEG_ARMOR: usize = 17;
    const NUMBER_OF_LONG_GUNS: usize = 15;
    const NUMBER_OF_MELEE_WEAPONS: usize = 17;
    const NUMBER_OF_RINGS: usize = 45;
    const NUMBER_OF_TRAITS: usize = 40;

    #[test]
    fn all_amulets_found() {
        assert_eq!(NUMBER_OF_AMULETS, amulets().len());
    }

    #[test]
    fn all_amulet_ids_unique() {
        let mut amulets = amulets();
        amulets.dedup_by_key(|x| x.id);
        assert_eq!(NUMBER_OF_AMULETS, amulets.len());
    }

    #[test]
    fn all_armor_sets_found() {
        assert_eq!(NUMBER_OF_ARMOR_SETS, armor_sets().len());
    }

    #[test]
    fn all_armor_set_ids_unique() {
        let mut armor_sets = armor_sets();
        armor_sets.dedup_by_key(|x| x.id);
        assert_eq!(NUMBER_OF_ARMOR_SETS, armor_sets.len());
    }

    #[test]
    fn all_body_armor_found() {
        assert_eq!(NUMBER_OF_BODY_ARMOR, body_armor().len());
    }

    #[test]
    fn all_body_armor_ids_unique() {
        let mut body_armor = body_armor();
        body_armor.dedup_by_key(|x| x.id);
        assert_eq!(NUMBER_OF_BODY_ARMOR, body_armor.len());
    }

    #[test]
    fn all_hand_guns_found() {
        assert_eq!(NUMBER_OF_HAND_GUNS, hand_guns().len());
    }

    #[test]
    fn all_hand_gun_ids_unique() {
        let mut hand_guns = hand_guns();
        hand_guns.dedup_by_key(|x| x.id);
        assert_eq!(NUMBER_OF_HAND_GUNS, hand_guns.len());
    }

    #[test]
    fn all_head_armor_found() {
        assert_eq!(NUMBER_OF_HEAD_ARMOR, head_armor().len());
    }

    #[test]
    fn all_head_armor_ids_unique() {
        let mut head_armor = head_armor();
        head_armor.dedup_by_key(|x| x.id);
        assert_eq!(NUMBER_OF_HEAD_ARMOR, head_armor.len());
    }

    #[test]
    fn all_emotes_found() {
        assert_eq!(NUMBER_OF_EMOTES, emotes().len());
    }

    #[test]
    fn all_emote_ids_unique() {
        let mut emotes = emotes();
        emotes.dedup_by_key(|x| x.id);
        assert_eq!(NUMBER_OF_EMOTES, emotes.len());
    }

    #[test]
    fn all_leg_armor_found() {
        assert_eq!(NUMBER_OF_LEG_ARMOR, leg_armor().len());
    }

    #[test]
    fn all_leg_armor_ids_unique() {
        let mut leg_armor = leg_armor();
        leg_armor.dedup_by_key(|x| x.id);
        assert_eq!(NUMBER_OF_LEG_ARMOR, leg_armor.len());
    }

    #[test]
    fn all_long_guns_found() {
        assert_eq!(NUMBER_OF_LONG_GUNS, long_guns().len());
    }

    #[test]
    fn all_long_gun_ids_unique() {
        let mut long_guns = long_guns();
        long_guns.dedup_by_key(|x| x.id);
        assert_eq!(NUMBER_OF_LONG_GUNS, long_guns.len());
    }

    #[test]
    fn all_melee_weapons_found() {
        assert_eq!(NUMBER_OF_MELEE_WEAPONS, melee_weapons().len());
    }

    #[test]
    fn all_melee_weapon_ids_unique() {
        let mut melee_weapons = melee_weapons();
        melee_weapons.dedup_by_key(|x| x.id);
        assert_eq!(NUMBER_OF_MELEE_WEAPONS, melee_weapons.len());
    }

    #[test]
    fn all_rings_found() {
        assert_eq!(NUMBER_OF_RINGS, rings().len());
    }

    #[test]
    fn all_ring_ids_unique() {
        let mut rings = rings();
        rings.dedup_by_key(|x| x.id);
        assert_eq!(NUMBER_OF_RINGS, rings.len());
    }

    #[test]
    fn all_traits_found() {
        assert_eq!(NUMBER_OF_TRAITS, remnant_traits().len());
    }

    #[test]
    fn all_trait_ids_unique() {
        let mut traits = remnant_traits();
        traits.dedup_by_key(|x| x.id);
        assert_eq!(NUMBER_OF_TRAITS, traits.len());
    }
}
