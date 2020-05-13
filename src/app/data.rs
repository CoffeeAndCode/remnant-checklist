use super::Entry;
use serde_derive::{Deserialize, Serialize};
use std::fmt::Display;

#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub enum World {
    Corsus,
    Earth,
    Labyrinth,
    Rhom,
    Ward13,
    Yaesha,
}

impl Default for World {
    fn default() -> World {
        World::Ward13
    }
}

impl Display for World {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let world_str = match self {
            World::Corsus => "corsus",
            World::Earth => "earth",
            World::Labyrinth => "labyrinth",
            World::Rhom => "rhom",
            World::Ward13 => "ward13",
            World::Yaesha => "yaesha",
        };
        write!(f, "{}", world_str)
    }
}

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

impl Display for DataType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            DataType::Amulet => "amulet",
            DataType::ArmorSet => "armor-set",
            DataType::BodyArmor => "body-armor",
            DataType::Emote => "emote",
            DataType::HandGun => "hand-gun",
            DataType::HeadArmor => "head-armor",
            DataType::LegArmor => "leg-armor",
            DataType::LongGun => "long-gun",
            DataType::MeleeWeapon => "melee-weapon",
            DataType::Ring => "ring",
            DataType::Trait => "trait",
        };
        write!(f, "{}", str)
    }
}

trait CsvDataSource<T> {
    fn data() -> &'static [u8];
    fn entries() -> Vec<Entry>;
    fn items() -> Vec<T>;
}

pub trait DataDisplay {
    fn icon(&self) -> char;
    fn label(&self) -> &'static str;
}

pub trait EntryCompatible {
    fn data_type() -> DataType;
    fn id(&self) -> u32;
    fn icon() -> char;
    fn name(&self) -> &str;
    fn url(&self) -> &str;
}

impl DataDisplay for DataType {
    fn icon(&self) -> char {
        match *self {
            DataType::Amulet => '💎',
            DataType::ArmorSet => '👘',
            DataType::BodyArmor => '🧥',
            DataType::Emote => '☺',
            DataType::HandGun => '🔫',
            DataType::HeadArmor => '💂',
            DataType::LegArmor => '🧦',
            DataType::LongGun => '⚒',
            DataType::MeleeWeapon => '🔱',
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

    #[serde(rename = "Url")]
    url: String,
}

#[derive(Debug, Deserialize)]
struct ArmorSet {
    #[serde(rename = "ID")]
    id: u32,

    #[serde(rename = "Name")]
    name: String,

    #[serde(rename = "Set Bonus")]
    set_bonus: String,

    #[serde(rename = "Url")]
    url: String,
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

    #[serde(rename = "Url")]
    url: String,
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

    #[serde(rename = "Url")]
    url: String,
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

    #[serde(rename = "Url")]
    url: String,
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

    #[serde(rename = "Url")]
    url: String,
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

    #[serde(rename = "Url")]
    url: String,
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

    #[serde(rename = "Url")]
    url: String,
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

    #[serde(rename = "Url")]
    url: String,
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

    #[serde(rename = "Url")]
    url: String,
}

#[derive(Debug, Deserialize)]
struct Trait {
    #[serde(rename = "Description")]
    description: String,

    #[serde(rename = "ID")]
    id: u32,

    #[serde(rename = "Name")]
    name: String,

    #[serde(rename = "Url")]
    url: String,
}

impl<'de, T> CsvDataSource<T> for T
where
    T: EntryCompatible + serde::de::DeserializeOwned,
{
    fn data() -> &'static [u8] {
        T::data_type().data()
    }

    fn entries() -> Vec<Entry> {
        <T as CsvDataSource<T>>::items()
            .into_iter()
            .map(Entry::from)
            .collect()
    }

    fn items() -> Vec<T> {
        let mut rdr = csv::Reader::from_reader(<T as CsvDataSource<T>>::data());
        let mut items: Vec<T> = rdr
            .deserialize()
            .filter_map(|t: Result<T, _>| t.ok())
            .collect();
        items.sort_unstable_by(|a, b| a.name().cmp(&b.name()));
        items
    }
}

impl EntryCompatible for Amulet {
    fn data_type() -> DataType {
        DataType::Amulet
    }

    fn icon() -> char {
        DataType::Amulet.icon()
    }

    fn id(&self) -> u32 {
        self.id
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn url(&self) -> &str {
        &self.url
    }
}

impl EntryCompatible for ArmorSet {
    fn data_type() -> DataType {
        DataType::ArmorSet
    }

    fn icon() -> char {
        DataType::ArmorSet.icon()
    }

    fn id(&self) -> u32 {
        self.id
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn url(&self) -> &str {
        &self.url
    }
}

impl EntryCompatible for BodyArmor {
    fn data_type() -> DataType {
        DataType::BodyArmor
    }

    fn icon() -> char {
        DataType::BodyArmor.icon()
    }

    fn id(&self) -> u32 {
        self.id
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn url(&self) -> &str {
        &self.url
    }
}

impl EntryCompatible for Emote {
    fn data_type() -> DataType {
        DataType::Emote
    }

    fn icon() -> char {
        DataType::Emote.icon()
    }

    fn id(&self) -> u32 {
        self.id
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn url(&self) -> &str {
        &self.url
    }
}

impl EntryCompatible for HandGun {
    fn data_type() -> DataType {
        DataType::HandGun
    }

    fn icon() -> char {
        DataType::HandGun.icon()
    }

    fn id(&self) -> u32 {
        self.id
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn url(&self) -> &str {
        &self.url
    }
}

impl EntryCompatible for HeadArmor {
    fn data_type() -> DataType {
        DataType::HeadArmor
    }

    fn icon() -> char {
        DataType::HeadArmor.icon()
    }

    fn id(&self) -> u32 {
        self.id
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn url(&self) -> &str {
        &self.url
    }
}

impl EntryCompatible for LegArmor {
    fn data_type() -> DataType {
        DataType::LegArmor
    }

    fn icon() -> char {
        DataType::LegArmor.icon()
    }

    fn id(&self) -> u32 {
        self.id
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn url(&self) -> &str {
        &self.url
    }
}

impl EntryCompatible for LongGun {
    fn data_type() -> DataType {
        DataType::LongGun
    }

    fn icon() -> char {
        DataType::LongGun.icon()
    }

    fn id(&self) -> u32 {
        self.id
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn url(&self) -> &str {
        &self.url
    }
}

impl EntryCompatible for MeleeWeapon {
    fn data_type() -> DataType {
        DataType::MeleeWeapon
    }

    fn icon() -> char {
        DataType::MeleeWeapon.icon()
    }

    fn id(&self) -> u32 {
        self.id
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn url(&self) -> &str {
        &self.url
    }
}

impl EntryCompatible for Ring {
    fn data_type() -> DataType {
        DataType::Ring
    }

    fn icon() -> char {
        DataType::Ring.icon()
    }

    fn id(&self) -> u32 {
        self.id
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn url(&self) -> &str {
        &self.url
    }
}

impl EntryCompatible for Trait {
    fn data_type() -> DataType {
        DataType::Trait
    }

    fn icon() -> char {
        DataType::Trait.icon()
    }

    fn id(&self) -> u32 {
        self.id
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn url(&self) -> &str {
        &self.url
    }
}

impl<T: EntryCompatible> From<T> for Entry {
    fn from(item: T) -> Self {
        Entry {
            completed: false,
            data_type: T::data_type(),
            icon: T::icon(),
            id: item.id(),
            name: String::from(item.name()),
            url: String::from(item.url()),
        }
    }
}

pub fn amulet_entries() -> Vec<Entry> {
    Amulet::entries()
}

pub fn armor_set_entries() -> Vec<Entry> {
    ArmorSet::entries()
}

pub fn body_armor_entries() -> Vec<Entry> {
    BodyArmor::entries()
}

pub fn emote_entries() -> Vec<Entry> {
    Emote::entries()
}

pub fn hand_gun_entries() -> Vec<Entry> {
    HandGun::entries()
}

pub fn head_armor_entries() -> Vec<Entry> {
    HeadArmor::entries()
}

pub fn leg_armor_entries() -> Vec<Entry> {
    LegArmor::entries()
}

pub fn long_gun_entries() -> Vec<Entry> {
    LongGun::entries()
}

pub fn melee_weapon_entries() -> Vec<Entry> {
    MeleeWeapon::entries()
}

pub fn remnant_trait_entries() -> Vec<Entry> {
    Trait::entries()
}

pub fn ring_entries() -> Vec<Entry> {
    Ring::entries()
}

#[cfg(test)]
mod tests {
    use super::*;

    const NUMBER_OF_AMULETS: usize = 23;
    const NUMBER_OF_ARMOR_SETS: usize = 17;
    const NUMBER_OF_BODY_ARMOR: usize = 16;
    const NUMBER_OF_HAND_GUNS: usize = 9;
    const NUMBER_OF_HEAD_ARMOR: usize = 18;
    const NUMBER_OF_EMOTES: usize = 15;
    const NUMBER_OF_LEG_ARMOR: usize = 17;
    const NUMBER_OF_LONG_GUNS: usize = 15;
    const NUMBER_OF_MELEE_WEAPONS: usize = 17;
    const NUMBER_OF_RINGS: usize = 47;
    const NUMBER_OF_TRAITS: usize = 40;

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
        assert_eq!("Amulet of Epicaricacy", amulets[0].name);
        assert_eq!("Amulet of Perseverance", amulets[1].name);
        assert_eq!("Vengeance Idol", amulets[NUMBER_OF_AMULETS - 1].name);
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
        assert_eq!("Void Set", armor_sets[NUMBER_OF_ARMOR_SETS - 1].name);
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
        assert_eq!("Void Carapace", body_armor[NUMBER_OF_BODY_ARMOR - 1].name);
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
        assert_eq!("Submachine Gun", hand_guns[NUMBER_OF_HAND_GUNS - 1].name);
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
        assert_eq!("Void Skull", head_armor[NUMBER_OF_HEAD_ARMOR - 1].name);
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
        assert_eq!("Void Greaves", leg_armor[NUMBER_OF_LEG_ARMOR - 1].name);
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
        assert_eq!("Assault Rifle", long_guns[0].name);
        assert_eq!("Beam Rifle", long_guns[1].name);
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
            "World Breaker",
            melee_weapons[NUMBER_OF_MELEE_WEAPONS - 1].name
        );
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
        assert_eq!("Stone Of Balance", rings[NUMBER_OF_RINGS - 1].name);
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
        assert_eq!("Bark Skin", traits[1].name);
        assert_eq!("World Walker", traits[NUMBER_OF_TRAITS - 1].name);
    }
}
