use super::storage::CompletedItem;
use super::Entry;
use serde_derive::{Deserialize, Serialize};
use std::fmt::Display;
use std::str::FromStr;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

#[derive(Clone, Debug, EnumIter, PartialEq)]
pub enum World {
    Any,
    Corsus,
    Earth,
    Labyrinth,
    Reisum,
    Rhom,
    RuralEarth,
    Ward13,
    Ward17,
    WardPrime,
    Yaesha,
}

impl World {
    pub fn from_param(str: &str) -> Result<Self, String> {
        match str {
            "any" => Ok(Self::Any),
            "corsus" => Ok(Self::Corsus),
            "earth" => Ok(Self::Earth),
            "labyrinth" => Ok(Self::Labyrinth),
            "reisum" => Ok(Self::Reisum),
            "rhom" => Ok(Self::Rhom),
            "ruralearth" => Ok(Self::RuralEarth),
            "ward13" => Ok(Self::Ward13),
            "ward17" => Ok(Self::Ward17),
            "wardprime" => Ok(Self::WardPrime),
            "yaesha" => Ok(Self::Yaesha),
            _ => Err(format!("unknown world: {}", str)),
        }
    }
}

impl Default for World {
    fn default() -> Self {
        Self::Ward13
    }
}

impl Display for World {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            Self::Any => "Any World",
            Self::Corsus => "Corsus",
            Self::Earth => "Earth",
            Self::Labyrinth => "Labyrinth",
            Self::Reisum => "Reisum",
            Self::Rhom => "Rhom",
            Self::RuralEarth => "Rural Earth",
            Self::Ward13 => "Ward 13",
            Self::Ward17 => "Ward 17",
            Self::WardPrime => "Ward Prime",
            Self::Yaesha => "Yaesha",
        };
        write!(f, "{}", str)
    }
}

#[derive(Clone, Deserialize, Serialize)]
pub enum ItemType {
    Amulet,
    ArmorSet,
    BodyArmor,
    Emote,
    HandGun,
    HeadArmor,
    LegArmor,
    LongGun,
    MeleeWeapon,
    Mod,
    Ring,
    Trait,
}

impl Display for ItemType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            Self::Amulet => "Amulet",
            Self::ArmorSet => "Armor Set",
            Self::BodyArmor => "Body Armor",
            Self::Emote => "Emote",
            Self::HandGun => "Hand Gun",
            Self::HeadArmor => "Head Armor",
            Self::LegArmor => "Leg Armor",
            Self::LongGun => "Long Gun",
            Self::MeleeWeapon => "Melee Weapon",
            Self::Mod => "Mod",
            Self::Ring => "Ring",
            Self::Trait => "Trait",
        };
        write!(f, "{}", str)
    }
}

pub trait CsvDataSource<T>
where
    T: EntryCompatible,
{
    fn entries() -> Vec<Entry>;
    fn items() -> Vec<T>;
    fn worlds(&self) -> Vec<World>;
}

pub trait EntryCompatible {
    const DATA_TYPE: ItemType;
    const RAW_DATA: &'static [u8];

    fn id(&self) -> u32;
    fn name(&self) -> &str;
    fn url(&self) -> &str;
    fn worlds_str(&self) -> &str;
}

pub trait UrlParam {
    fn url_slug(self) -> &'static str;
}

impl UrlParam for ItemType {
    fn url_slug(self) -> &'static str {
        match self {
            Self::Amulet => "amulet",
            Self::ArmorSet => "armor-set",
            Self::BodyArmor => "body-armor",
            Self::Emote => "emote",
            Self::HandGun => "hand-gun",
            Self::HeadArmor => "head-armor",
            Self::LegArmor => "leg-armor",
            Self::LongGun => "long-gun",
            Self::MeleeWeapon => "melee-weapon",
            Self::Mod => "mod",
            Self::Ring => "ring",
            Self::Trait => "trait",
        }
    }
}

impl UrlParam for World {
    fn url_slug(self) -> &'static str {
        match self {
            Self::Any => "any",
            Self::Corsus => "corsus",
            Self::Earth => "earth",
            Self::Labyrinth => "labyrinth",
            Self::Reisum => "reisum",
            Self::Rhom => "rhom",
            Self::RuralEarth => "ruralearth",
            Self::Ward13 => "ward13",
            Self::Ward17 => "ward17",
            Self::WardPrime => "wardprime",
            Self::Yaesha => "yaesha",
        }
    }
}

#[derive(Deserialize)]
struct Amulet {
    #[serde(rename = "Description")]
    _description: Option<String>,

    #[serde(rename = "ID")]
    id: u32,

    #[serde(rename = "Location & Crafting")]
    _location: Option<String>,

    #[serde(rename = "Name")]
    name: String,

    #[serde(rename = "Url")]
    url: String,

    #[serde(rename = "Worlds")]
    worlds_str: String,
}

#[derive(Deserialize)]
struct ArmorSet {
    #[serde(rename = "ID")]
    id: u32,

    #[serde(rename = "Name")]
    name: String,

    #[serde(rename = "Set Bonus")]
    _set_bonus: String,

    #[serde(rename = "Url")]
    url: String,

    #[serde(rename = "Worlds")]
    worlds_str: String,
}

#[derive(Deserialize)]
struct BodyArmor {
    #[serde(rename = "Armor")]
    _armor: Option<f32>,

    #[serde(rename = "Armor Skill")]
    _armor_skill: String,

    #[serde(rename = "Bleed")]
    _bleed: Option<f32>,

    #[serde(rename = "Corrosive")]
    _corrosive: Option<f32>,

    #[serde(rename = "Fire")]
    _fire: Option<f32>,

    #[serde(rename = "Radiation")]
    _radiation: Option<f32>,

    #[serde(rename = "Rot")]
    _rot: Option<f32>,

    #[serde(rename = "Shock")]
    _shock: Option<f32>,

    #[serde(rename = "Weight")]
    _weight: Option<f32>,

    #[serde(rename = "ID")]
    id: u32,

    #[serde(rename = "Name")]
    name: String,

    #[serde(rename = "Url")]
    url: String,

    #[serde(rename = "Worlds")]
    worlds_str: String,
}

#[derive(Deserialize)]
struct Emote {
    #[serde(rename = "Description")]
    _description: String,

    #[serde(rename = "ID")]
    id: u32,

    #[serde(rename = "Location")]
    _location: String,

    #[serde(rename = "Name")]
    name: String,

    #[serde(rename = "Url")]
    url: String,

    #[serde(rename = "Worlds")]
    worlds_str: String,
}

#[derive(Deserialize)]
struct HandGun {
    #[serde(rename = "Base Damage")]
    _base_damage: u32,

    #[serde(rename = "Crit Chance")]
    _crit_chance: u32,

    #[serde(rename = "ID")]
    id: u32,

    #[serde(rename = "Ideal Range")]
    _ideal_range: u32,

    #[serde(rename = "Magazine")]
    _magazine: u32,

    #[serde(rename = "Max Ammo")]
    _max_ammo: u32,

    #[serde(rename = "Max Damage")]
    _max_damage: Option<u32>,

    #[serde(rename = "Name")]
    name: String,

    #[serde(rename = "RPS")]
    _rps: f32,

    #[serde(rename = "Url")]
    url: String,

    #[serde(rename = "Worlds")]
    worlds_str: String,
}

#[derive(Deserialize)]
struct HeadArmor {
    #[serde(rename = "Armor")]
    _armor: Option<f32>,

    #[serde(rename = "Armor Skill")]
    _armor_skill: String,

    #[serde(rename = "Bleed")]
    _bleed: Option<f32>,

    #[serde(rename = "Corrosive")]
    _corrosive: Option<f32>,

    #[serde(rename = "Fire")]
    _fire: Option<f32>,

    #[serde(rename = "Radiation")]
    _radiation: Option<f32>,

    #[serde(rename = "Rot")]
    _rot: Option<f32>,

    #[serde(rename = "Shock")]
    _shock: Option<f32>,

    #[serde(rename = "Weight")]
    _weight: Option<f32>,

    #[serde(rename = "ID")]
    id: u32,

    #[serde(rename = "Name")]
    name: String,

    #[serde(rename = "Url")]
    url: String,

    #[serde(rename = "Worlds")]
    worlds_str: String,
}

#[derive(Deserialize)]
struct LegArmor {
    #[serde(rename = "Armor")]
    _armor: Option<f32>,

    #[serde(rename = "Armor Skill")]
    _armor_skill: String,

    #[serde(rename = "Bleed")]
    _bleed: Option<f32>,

    #[serde(rename = "Corrosive")]
    _corrosive: Option<f32>,

    #[serde(rename = "Fire")]
    _fire: Option<f32>,

    #[serde(rename = "Radiation")]
    _radiation: Option<f32>,

    #[serde(rename = "Rot")]
    _rot: Option<f32>,

    #[serde(rename = "Shock")]
    _shock: Option<f32>,

    #[serde(rename = "Weight")]
    _weight: Option<f32>,

    #[serde(rename = "ID")]
    id: u32,

    #[serde(rename = "Name")]
    name: String,

    #[serde(rename = "Url")]
    url: String,

    #[serde(rename = "Worlds")]
    worlds_str: String,
}

#[derive(Deserialize)]
struct LongGun {
    #[serde(rename = "Base Damage")]
    _base_damage: u32,

    #[serde(rename = "Crit Chance")]
    _crit_chance: u32,

    #[serde(rename = "ID")]
    id: u32,

    #[serde(rename = "Ideal Range")]
    _ideal_range: u32,

    #[serde(rename = "Magazine")]
    _magazine: u32,

    #[serde(rename = "Max Ammo")]
    _max_ammo: u32,

    #[serde(rename = "Max Damage")]
    _max_damage: Option<u32>,

    #[serde(rename = "Name")]
    name: String,

    #[serde(rename = "RPS")]
    _rps: f32,

    #[serde(rename = "Url")]
    url: String,

    #[serde(rename = "Worlds")]
    worlds_str: String,
}

#[derive(Deserialize)]
struct MeleeWeapon {
    #[serde(rename = "Base Damage")]
    _base_damage: u32,

    #[serde(rename = "ID")]
    id: u32,

    #[serde(rename = "Max Damage")]
    _max_damage: Option<u32>,

    #[serde(rename = "Name")]
    name: String,

    #[serde(rename = "Weapon Mod")]
    _weapon_mod: Option<String>,

    #[serde(rename = "Url")]
    url: String,

    #[serde(rename = "Worlds")]
    worlds_str: String,
}

#[derive(Deserialize)]
struct Mod {
    #[serde(rename = "ID")]
    id: u32,

    #[serde(rename = "Name")]
    name: String,

    #[serde(rename = "Url")]
    url: String,

    #[serde(rename = "Worlds")]
    worlds_str: String,
}

#[derive(Deserialize)]
struct Ring {
    #[serde(rename = "Description")]
    _description: String,

    #[serde(rename = "ID")]
    id: u32,

    #[serde(rename = "Name")]
    name: String,

    #[serde(rename = "Location")]
    _location: String,

    #[serde(rename = "Url")]
    url: String,

    #[serde(rename = "Worlds")]
    worlds_str: String,
}

#[derive(Deserialize)]
struct Trait {
    #[serde(rename = "Description")]
    _description: String,

    #[serde(rename = "ID")]
    id: u32,

    #[serde(rename = "Name")]
    name: String,

    #[serde(rename = "Url")]
    url: String,

    #[serde(rename = "Worlds")]
    worlds_str: String,
}

impl<'de, T> CsvDataSource<T> for T
where
    T: EntryCompatible + serde::de::DeserializeOwned,
{
    fn entries() -> Vec<Entry> {
        <Self as CsvDataSource<Self>>::items()
            .into_iter()
            .map(Entry::from)
            .collect()
    }

    fn items() -> Vec<T> {
        let mut rdr = csv::Reader::from_reader(Self::RAW_DATA);
        let mut items: Vec<Self> = rdr.deserialize().filter_map(Result::ok).collect();
        items.sort_by_key(|a| a.name().to_string());
        items
    }

    fn worlds(&self) -> Vec<World> {
        if self.worlds_str().trim().is_empty() {
            return vec![World::Any];
        }
        if self.worlds_str().trim() == "Any" {
            return World::iter().collect();
        }

        let mut worlds: Vec<World> = self
            .worlds_str()
            .split(',')
            .map(str::trim)
            .map(World::from_str)
            .filter_map(Result::ok)
            .collect();
        worlds.push(World::Any);
        worlds
    }
}

impl FromStr for World {
    type Err = String;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value {
            "Corsus" => Ok(Self::Corsus),
            "Earth" => Ok(Self::Earth),
            "Labyrinth" => Ok(Self::Labyrinth),
            "Reisum" => Ok(Self::Reisum),
            "Rhom" => Ok(Self::Rhom),
            "Rural Earth" => Ok(Self::RuralEarth),
            "Ward 13" => Ok(Self::Ward13),
            "Ward 17" => Ok(Self::Ward17),
            "Ward Prime" => Ok(Self::WardPrime),
            "Yaesha" => Ok(Self::Yaesha),
            _ => Err(format!("Invalid world found: {}", value)),
        }
    }
}

impl EntryCompatible for Amulet {
    const DATA_TYPE: ItemType = ItemType::Amulet;
    const RAW_DATA: &'static [u8] = include_bytes!("../data/amulets.csv");

    fn id(&self) -> u32 {
        self.id
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn url(&self) -> &str {
        &self.url
    }

    fn worlds_str(&self) -> &str {
        &self.worlds_str
    }
}

impl EntryCompatible for ArmorSet {
    const DATA_TYPE: ItemType = ItemType::ArmorSet;
    const RAW_DATA: &'static [u8] = include_bytes!("../data/armor_sets.csv");

    fn id(&self) -> u32 {
        self.id
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn url(&self) -> &str {
        &self.url
    }

    fn worlds_str(&self) -> &str {
        &self.worlds_str
    }
}

impl EntryCompatible for BodyArmor {
    const DATA_TYPE: ItemType = ItemType::BodyArmor;
    const RAW_DATA: &'static [u8] = include_bytes!("../data/body_armor.csv");

    fn id(&self) -> u32 {
        self.id
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn url(&self) -> &str {
        &self.url
    }

    fn worlds_str(&self) -> &str {
        &self.worlds_str
    }
}

impl EntryCompatible for Emote {
    const DATA_TYPE: ItemType = ItemType::Emote;
    const RAW_DATA: &'static [u8] = include_bytes!("../data/emotes.csv");

    fn id(&self) -> u32 {
        self.id
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn url(&self) -> &str {
        &self.url
    }

    fn worlds_str(&self) -> &str {
        &self.worlds_str
    }
}

impl EntryCompatible for HandGun {
    const DATA_TYPE: ItemType = ItemType::HandGun;
    const RAW_DATA: &'static [u8] = include_bytes!("../data/hand_guns.csv");

    fn id(&self) -> u32 {
        self.id
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn url(&self) -> &str {
        &self.url
    }

    fn worlds_str(&self) -> &str {
        &self.worlds_str
    }
}

impl EntryCompatible for HeadArmor {
    const DATA_TYPE: ItemType = ItemType::HeadArmor;
    const RAW_DATA: &'static [u8] = include_bytes!("../data/head_armor.csv");

    fn id(&self) -> u32 {
        self.id
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn url(&self) -> &str {
        &self.url
    }

    fn worlds_str(&self) -> &str {
        &self.worlds_str
    }
}

impl EntryCompatible for LegArmor {
    const DATA_TYPE: ItemType = ItemType::LegArmor;
    const RAW_DATA: &'static [u8] = include_bytes!("../data/leg_armor.csv");

    fn id(&self) -> u32 {
        self.id
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn url(&self) -> &str {
        &self.url
    }

    fn worlds_str(&self) -> &str {
        &self.worlds_str
    }
}

impl EntryCompatible for LongGun {
    const DATA_TYPE: ItemType = ItemType::LongGun;
    const RAW_DATA: &'static [u8] = include_bytes!("../data/long_guns.csv");

    fn id(&self) -> u32 {
        self.id
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn url(&self) -> &str {
        &self.url
    }

    fn worlds_str(&self) -> &str {
        &self.worlds_str
    }
}

impl EntryCompatible for MeleeWeapon {
    const DATA_TYPE: ItemType = ItemType::MeleeWeapon;
    const RAW_DATA: &'static [u8] = include_bytes!("../data/melee_weapons.csv");

    fn id(&self) -> u32 {
        self.id
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn url(&self) -> &str {
        &self.url
    }

    fn worlds_str(&self) -> &str {
        &self.worlds_str
    }
}

impl EntryCompatible for Mod {
    const DATA_TYPE: ItemType = ItemType::Mod;
    const RAW_DATA: &'static [u8] = include_bytes!("../data/mods.csv");

    fn id(&self) -> u32 {
        self.id
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn url(&self) -> &str {
        &self.url
    }

    fn worlds_str(&self) -> &str {
        &self.worlds_str
    }
}

impl EntryCompatible for Ring {
    const DATA_TYPE: ItemType = ItemType::Ring;
    const RAW_DATA: &'static [u8] = include_bytes!("../data/rings.csv");

    fn id(&self) -> u32 {
        self.id
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn url(&self) -> &str {
        &self.url
    }

    fn worlds_str(&self) -> &str {
        &self.worlds_str
    }
}

impl EntryCompatible for Trait {
    const DATA_TYPE: ItemType = ItemType::Trait;
    const RAW_DATA: &'static [u8] = include_bytes!("../data/traits.csv");

    fn id(&self) -> u32 {
        self.id
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn url(&self) -> &str {
        &self.url
    }

    fn worlds_str(&self) -> &str {
        &self.worlds_str
    }
}

impl<T: CsvDataSource<T> + EntryCompatible> From<T> for Entry {
    fn from(item: T) -> Self {
        Self {
            completed: false,
            data_type: T::DATA_TYPE,
            id: item.id(),
            name: String::from(item.name()),
            url: String::from(item.url()),
            worlds: item.worlds(),
        }
    }
}

pub fn amulet_entries(defaults: &[CompletedItem]) -> Vec<Entry> {
    let mut entries = Amulet::entries();

    #[allow(clippy::blocks_in_if_conditions)]
    for entry in &mut entries {
        if defaults
            .iter()
            .any(|default| default.id == entry.id && matches!(default.data_type, ItemType::Amulet))
        {
            entry.completed = true;
        }
    }
    entries
}

pub fn armor_set_entries(defaults: &[CompletedItem]) -> Vec<Entry> {
    let mut entries = ArmorSet::entries();

    #[allow(clippy::blocks_in_if_conditions)]
    for entry in &mut entries {
        if defaults.iter().any(|default| {
            default.id == entry.id && matches!(default.data_type, ItemType::ArmorSet)
        }) {
            entry.completed = true;
        }
    }
    entries
}

pub fn body_armor_entries(defaults: &[CompletedItem]) -> Vec<Entry> {
    let mut entries = BodyArmor::entries();

    #[allow(clippy::blocks_in_if_conditions)]
    for entry in &mut entries {
        if defaults.iter().any(|default| {
            default.id == entry.id && matches!(default.data_type, ItemType::BodyArmor)
        }) {
            entry.completed = true;
        }
    }
    entries
}

pub fn emote_entries(defaults: &[CompletedItem]) -> Vec<Entry> {
    let mut entries = Emote::entries();

    #[allow(clippy::blocks_in_if_conditions)]
    for entry in &mut entries {
        if defaults
            .iter()
            .any(|default| default.id == entry.id && matches!(default.data_type, ItemType::Emote))
        {
            entry.completed = true;
        }
    }
    entries
}

pub fn hand_gun_entries(defaults: &[CompletedItem]) -> Vec<Entry> {
    let mut entries = HandGun::entries();

    #[allow(clippy::blocks_in_if_conditions)]
    for entry in &mut entries {
        if defaults
            .iter()
            .any(|default| default.id == entry.id && matches!(default.data_type, ItemType::HandGun))
        {
            entry.completed = true;
        }
    }
    entries
}

pub fn head_armor_entries(defaults: &[CompletedItem]) -> Vec<Entry> {
    let mut entries = HeadArmor::entries();

    #[allow(clippy::blocks_in_if_conditions)]
    for entry in &mut entries {
        if defaults.iter().any(|default| {
            default.id == entry.id && matches!(default.data_type, ItemType::HeadArmor)
        }) {
            entry.completed = true;
        }
    }
    entries
}

pub fn leg_armor_entries(defaults: &[CompletedItem]) -> Vec<Entry> {
    let mut entries = LegArmor::entries();

    #[allow(clippy::blocks_in_if_conditions)]
    for entry in &mut entries {
        if defaults.iter().any(|default| {
            default.id == entry.id && matches!(default.data_type, ItemType::LegArmor)
        }) {
            entry.completed = true;
        }
    }
    entries
}

pub fn long_gun_entries(defaults: &[CompletedItem]) -> Vec<Entry> {
    let mut entries = LongGun::entries();

    #[allow(clippy::blocks_in_if_conditions)]
    for entry in &mut entries {
        if defaults
            .iter()
            .any(|default| default.id == entry.id && matches!(default.data_type, ItemType::LongGun))
        {
            entry.completed = true;
        }
    }
    entries
}

pub fn melee_weapon_entries(defaults: &[CompletedItem]) -> Vec<Entry> {
    let mut entries = MeleeWeapon::entries();

    #[allow(clippy::blocks_in_if_conditions)]
    for entry in &mut entries {
        if defaults.iter().any(|default| {
            default.id == entry.id && matches!(default.data_type, ItemType::MeleeWeapon)
        }) {
            entry.completed = true;
        }
    }
    entries
}

pub fn mod_entries(defaults: &[CompletedItem]) -> Vec<Entry> {
    let mut entries = Mod::entries();

    #[allow(clippy::blocks_in_if_conditions)]
    for entry in &mut entries {
        if defaults
            .iter()
            .any(|default| default.id == entry.id && matches!(default.data_type, ItemType::Mod))
        {
            entry.completed = true;
        }
    }
    entries
}

pub fn remnant_trait_entries(defaults: &[CompletedItem]) -> Vec<Entry> {
    let mut entries = Trait::entries();

    #[allow(clippy::blocks_in_if_conditions)]
    for entry in &mut entries {
        if defaults
            .iter()
            .any(|default| default.id == entry.id && matches!(default.data_type, ItemType::Trait))
        {
            entry.completed = true;
        }
    }
    entries
}

pub fn ring_entries(defaults: &[CompletedItem]) -> Vec<Entry> {
    let mut entries = Ring::entries();

    #[allow(clippy::blocks_in_if_conditions)]
    for entry in &mut entries {
        if defaults
            .iter()
            .any(|default| default.id == entry.id && matches!(default.data_type, ItemType::Ring))
        {
            entry.completed = true;
        }
    }
    entries
}

#[cfg(test)]
#[allow(clippy::wildcard_imports)]
mod tests {
    use super::*;

    const NUMBER_OF_AMULETS: usize = 37;
    const NUMBER_OF_ARMOR_SETS: usize = 19;
    const NUMBER_OF_BODY_ARMOR: usize = 18;
    const NUMBER_OF_HAND_GUNS: usize = 12;
    const NUMBER_OF_HEAD_ARMOR: usize = 20;
    const NUMBER_OF_EMOTES: usize = 15;
    const NUMBER_OF_LEG_ARMOR: usize = 18;
    const NUMBER_OF_LONG_GUNS: usize = 17;
    const NUMBER_OF_MELEE_WEAPONS: usize = 20;
    const NUMBER_OF_MODS: usize = 40;
    const NUMBER_OF_RINGS: usize = 74;
    const NUMBER_OF_TRAITS: usize = 50;

    #[test]
    fn all_amulets_found() {
        assert_eq!(NUMBER_OF_AMULETS, Amulet::items().len());
    }

    #[test]
    fn all_amulet_ids_unique() {
        let mut amulets = Amulet::items();
        amulets.dedup_by_key(|x| x.id);
        assert_eq!(NUMBER_OF_AMULETS, amulets.len());
    }

    #[test]
    fn amulets_are_sorted_alphabetically() {
        let amulets = Amulet::items();
        assert_eq!("Abrasive Amulet", amulets[0].name);
        assert_eq!("Amulet of Epicaricacy", amulets[1].name);
        assert_eq!("White Rose", amulets[NUMBER_OF_AMULETS - 1].name);
    }

    #[test]
    fn all_armor_sets_found() {
        assert_eq!(NUMBER_OF_ARMOR_SETS, ArmorSet::items().len());
    }

    #[test]
    fn all_armor_set_ids_unique() {
        let mut armor_sets = ArmorSet::items();
        armor_sets.dedup_by_key(|x| x.id);
        assert_eq!(NUMBER_OF_ARMOR_SETS, armor_sets.len());
    }

    #[test]
    fn armor_sets_are_sorted_alphabetically() {
        let armor_sets = ArmorSet::items();
        assert_eq!("Adventurer Set", armor_sets[0].name);
        assert_eq!("Akari Set", armor_sets[1].name);
        assert_eq!("Warlord's Set", armor_sets[NUMBER_OF_ARMOR_SETS - 1].name);
    }

    #[test]
    fn all_body_armor_found() {
        assert_eq!(NUMBER_OF_BODY_ARMOR, BodyArmor::items().len());
    }

    #[test]
    fn all_body_armor_ids_unique() {
        let mut body_armor = BodyArmor::items();
        body_armor.dedup_by_key(|x| x.id);
        assert_eq!(NUMBER_OF_BODY_ARMOR, body_armor.len());
    }

    #[test]
    fn body_armor_is_sorted_alphabetically() {
        let body_armor = BodyArmor::items();
        assert_eq!("Adventurer Tunic", body_armor[0].name);
        assert_eq!("Akari Garb", body_armor[1].name);
        assert_eq!("Warlord Armor", body_armor[NUMBER_OF_BODY_ARMOR - 1].name);
    }

    #[test]
    fn all_hand_guns_found() {
        assert_eq!(NUMBER_OF_HAND_GUNS, HandGun::items().len());
    }

    #[test]
    fn all_hand_gun_ids_unique() {
        let mut hand_guns = HandGun::items();
        hand_guns.dedup_by_key(|x| x.id);
        assert_eq!(NUMBER_OF_HAND_GUNS, hand_guns.len());
    }

    #[test]
    fn hand_guns_are_sorted_alphabetically() {
        let hand_guns = HandGun::items();
        assert_eq!("Curse of the Jungle God", hand_guns[0].name);
        assert_eq!("Defiler", hand_guns[1].name);
        assert_eq!("Twin Shot", hand_guns[NUMBER_OF_HAND_GUNS - 1].name);
    }

    #[test]
    fn all_head_armor_found() {
        assert_eq!(NUMBER_OF_HEAD_ARMOR, HeadArmor::items().len());
    }

    #[test]
    fn all_head_armor_ids_unique() {
        let mut head_armor = HeadArmor::items();
        head_armor.dedup_by_key(|x| x.id);
        assert_eq!(NUMBER_OF_HEAD_ARMOR, head_armor.len());
    }

    #[test]
    fn head_armor_is_sorted_alphabetically() {
        let head_armor = HeadArmor::items();
        assert_eq!("Adventurer Goggles", head_armor[0].name);
        assert_eq!("Akari Mask", head_armor[1].name);
        assert_eq!("Warlord Skull", head_armor[NUMBER_OF_HEAD_ARMOR - 1].name);
    }

    #[test]
    fn all_emotes_found() {
        assert_eq!(NUMBER_OF_EMOTES, Emote::items().len());
    }

    #[test]
    fn all_emote_ids_unique() {
        let mut emotes = Emote::items();
        emotes.dedup_by_key(|x| x.id);
        assert_eq!(NUMBER_OF_EMOTES, emotes.len());
    }

    #[test]
    fn emotes_are_sorted_alphabetically() {
        let emotes = Emote::items();
        assert_eq!("Beckon Emote", emotes[0].name);
        assert_eq!("Cheer Emote", emotes[1].name);
        assert_eq!("Yes Emote", emotes[NUMBER_OF_EMOTES - 1].name);
    }

    #[test]
    fn all_leg_armor_found() {
        assert_eq!(NUMBER_OF_LEG_ARMOR, LegArmor::items().len());
    }

    #[test]
    fn all_leg_armor_ids_unique() {
        let mut leg_armor = LegArmor::items();
        leg_armor.dedup_by_key(|x| x.id);
        assert_eq!(NUMBER_OF_LEG_ARMOR, leg_armor.len());
    }

    #[test]
    fn leg_armor_is_sorted_alphabetically() {
        let leg_armor = LegArmor::items();
        assert_eq!("Adventurer Leggings", leg_armor[0].name);
        assert_eq!("Akari Leggings", leg_armor[1].name);
        assert_eq!("Warlord Boots", leg_armor[NUMBER_OF_LEG_ARMOR - 1].name);
    }

    #[test]
    fn all_long_guns_found() {
        assert_eq!(NUMBER_OF_LONG_GUNS, LongGun::items().len());
    }

    #[test]
    fn all_long_gun_ids_unique() {
        let mut long_guns = LongGun::items();
        long_guns.dedup_by_key(|x| x.id);
        assert_eq!(NUMBER_OF_LONG_GUNS, long_guns.len());
    }

    #[test]
    fn long_guns_are_sorted_alphabetically() {
        let long_guns = LongGun::items();
        assert_eq!("Alternator", long_guns[0].name);
        assert_eq!("Assault Rifle", long_guns[1].name);
        assert_eq!("Sporebloom", long_guns[NUMBER_OF_LONG_GUNS - 1].name);
    }

    #[test]
    fn all_melee_weapons_found() {
        assert_eq!(NUMBER_OF_MELEE_WEAPONS, MeleeWeapon::items().len());
    }

    #[test]
    fn all_melee_weapon_ids_unique() {
        let mut melee_weapons = MeleeWeapon::items();
        melee_weapons.dedup_by_key(|x| x.id);
        assert_eq!(NUMBER_OF_MELEE_WEAPONS, melee_weapons.len());
    }

    #[test]
    fn melee_weapons_are_sorted_alphabetically() {
        let melee_weapons = MeleeWeapon::items();
        assert_eq!("Blade of Adventure", melee_weapons[0].name);
        assert_eq!("Butchers Flail", melee_weapons[1].name);
        assert_eq!(
            "World's Edge",
            melee_weapons[NUMBER_OF_MELEE_WEAPONS - 1].name
        );
    }

    #[test]
    fn all_mods_found() {
        assert_eq!(NUMBER_OF_MODS, Mod::items().len());
    }

    #[test]
    fn all_mod_ids_unique() {
        let mut mods = Mod::items();
        mods.dedup_by_key(|x| x.id);
        assert_eq!(NUMBER_OF_MODS, mods.len());
    }

    #[test]
    fn mods_are_sorted_alphabetically() {
        let mods = Mod::items();
        assert_eq!("Banish", mods[0].name);
        assert_eq!("Beckon", mods[1].name);
        assert_eq!("Wildfire Shot", mods[NUMBER_OF_MODS - 1].name);
    }

    #[test]
    fn all_rings_found() {
        assert_eq!(NUMBER_OF_RINGS, Ring::items().len());
    }

    #[test]
    fn all_ring_ids_unique() {
        let mut rings = Ring::items();
        rings.dedup_by_key(|x| x.id);
        assert_eq!(NUMBER_OF_RINGS, rings.len());
    }

    #[test]
    fn rings_are_sorted_alphabetically() {
        let rings = Ring::items();
        assert_eq!("Aggressor's Bane", rings[0].name);
        assert_eq!("Akari War Band", rings[1].name);
        assert_eq!("Volatile Gem", rings[NUMBER_OF_RINGS - 1].name);
    }

    #[test]
    fn all_traits_found() {
        assert_eq!(NUMBER_OF_TRAITS, Trait::items().len());
    }

    #[test]
    fn all_trait_ids_unique() {
        let mut traits = Trait::items();
        traits.dedup_by_key(|x| x.id);
        assert_eq!(NUMBER_OF_TRAITS, traits.len());
    }

    #[test]
    fn traits_are_sorted_alphabetically() {
        let traits = Trait::items();
        assert_eq!("Arcane Strike", traits[0].name);
        assert_eq!("Armor Piercer", traits[1].name);
        assert_eq!("World Walker", traits[NUMBER_OF_TRAITS - 1].name);
    }

    #[test]
    fn worlds_str_is_converted_to_a_vec_of_worlds() {
        let amulet = Amulet {
            _description: None,
            id: 1,
            _location: None,
            name: String::from("example"),
            url: String::from("www.example.com"),
            worlds_str: String::from("Ward 17,Earth"),
        };

        assert_eq!(
            vec![World::Ward17, World::Earth, World::Any],
            amulet.worlds()
        );
    }

    #[test]
    fn worlds_str_ignores_invalid_input() {
        let amulet = Amulet {
            _description: None,
            id: 1,
            _location: None,
            name: String::from("example"),
            url: String::from("www.example.com"),
            worlds_str: String::from("Nope,Earth,Invalid"),
        };

        assert_eq!(vec![World::Earth, World::Any], amulet.worlds());
    }

    #[test]
    fn any_worlds_str_is_converted_to_a_vec_of_all_worlds() {
        let amulet = Amulet {
            _description: None,
            id: 1,
            _location: None,
            name: String::from("example"),
            url: String::from("www.example.com"),
            worlds_str: String::from("Any"),
        };

        assert_eq!(
            vec![
                World::Any,
                World::Corsus,
                World::Earth,
                World::Labyrinth,
                World::Reisum,
                World::Rhom,
                World::RuralEarth,
                World::Ward13,
                World::Ward17,
                World::WardPrime,
                World::Yaesha,
            ],
            amulet.worlds()
        );
    }
}
