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

impl From<Amulet> for Entry {
    fn from(amulet: Amulet) -> Self {
        Entry {
            completed: false,
            data_type: DataType::Amulet,
            icon: DataType::Amulet.icon(),
            name: amulet.name,
            url: amulet.url,
        }
    }
}

impl From<ArmorSet> for Entry {
    fn from(armor_set: ArmorSet) -> Self {
        Entry {
            completed: false,
            data_type: DataType::ArmorSet,
            icon: DataType::ArmorSet.icon(),
            name: armor_set.name,
            url: armor_set.url,
        }
    }
}

impl From<BodyArmor> for Entry {
    fn from(body_armor: BodyArmor) -> Self {
        Entry {
            completed: false,
            data_type: DataType::BodyArmor,
            icon: DataType::BodyArmor.icon(),
            name: body_armor.name,
            url: body_armor.url,
        }
    }
}

impl From<Emote> for Entry {
    fn from(emote: Emote) -> Self {
        Entry {
            completed: false,
            data_type: DataType::Emote,
            icon: DataType::Emote.icon(),
            name: emote.name,
            url: emote.url,
        }
    }
}

impl From<HandGun> for Entry {
    fn from(hand_gun: HandGun) -> Self {
        Entry {
            completed: false,
            data_type: DataType::HandGun,
            icon: DataType::HandGun.icon(),
            name: hand_gun.name,
            url: hand_gun.url,
        }
    }
}

impl From<HeadArmor> for Entry {
    fn from(head_armor: HeadArmor) -> Self {
        Entry {
            completed: false,
            data_type: DataType::HeadArmor,
            icon: DataType::HeadArmor.icon(),
            name: head_armor.name,
            url: head_armor.url,
        }
    }
}

impl From<LegArmor> for Entry {
    fn from(leg_armor: LegArmor) -> Self {
        Entry {
            completed: false,
            data_type: DataType::LegArmor,
            icon: DataType::LegArmor.icon(),
            name: leg_armor.name,
            url: leg_armor.url,
        }
    }
}

impl From<LongGun> for Entry {
    fn from(long_gun: LongGun) -> Self {
        Entry {
            completed: false,
            data_type: DataType::LongGun,
            icon: DataType::LongGun.icon(),
            name: long_gun.name,
            url: long_gun.url,
        }
    }
}

impl From<MeleeWeapon> for Entry {
    fn from(melee_weapon: MeleeWeapon) -> Self {
        Entry {
            completed: false,
            data_type: DataType::MeleeWeapon,
            icon: DataType::MeleeWeapon.icon(),
            name: melee_weapon.name,
            url: melee_weapon.url,
        }
    }
}

impl From<Ring> for Entry {
    fn from(ring: Ring) -> Self {
        Entry {
            completed: false,
            data_type: DataType::MeleeWeapon,
            icon: DataType::Ring.icon(),
            name: ring.name,
            url: ring.url,
        }
    }
}

impl From<Trait> for Entry {
    fn from(remnant_trait: Trait) -> Self {
        Entry {
            completed: false,
            data_type: DataType::Trait,
            icon: DataType::Trait.icon(),
            name: remnant_trait.name,
            url: remnant_trait.url,
        }
    }
}

fn amulets() -> Vec<Amulet> {
    let mut rdr = csv::Reader::from_reader(DataType::Amulet.data());
    let mut amulets: Vec<Amulet> = rdr
        .deserialize()
        .filter_map(|t: Result<Amulet, _>| t.ok())
        .collect();
    amulets.sort_unstable_by(|a, b| a.name.cmp(&b.name));
    amulets
}

pub fn amulet_entries() -> Vec<Entry> {
    amulets().into_iter().map(Entry::from).collect()
}

fn armor_sets() -> Vec<ArmorSet> {
    let mut rdr = csv::Reader::from_reader(DataType::ArmorSet.data());
    let mut armor_sets: Vec<ArmorSet> = rdr
        .deserialize()
        .filter_map(|t: Result<ArmorSet, _>| t.ok())
        .collect();
    armor_sets.sort_unstable_by(|a, b| a.name.cmp(&b.name));
    armor_sets
}

pub fn armor_set_entries() -> Vec<Entry> {
    armor_sets().into_iter().map(Entry::from).collect()
}

fn body_armor() -> Vec<BodyArmor> {
    let mut rdr = csv::Reader::from_reader(DataType::BodyArmor.data());
    let mut body_armor: Vec<BodyArmor> = rdr
        .deserialize()
        .filter_map(|t: Result<BodyArmor, _>| t.ok())
        .collect();
    body_armor.sort_unstable_by(|a, b| a.name.cmp(&b.name));
    body_armor
}

pub fn body_armor_entries() -> Vec<Entry> {
    body_armor().into_iter().map(Entry::from).collect()
}

fn emotes() -> Vec<Emote> {
    let mut rdr = csv::Reader::from_reader(DataType::Emote.data());
    let mut emotes: Vec<Emote> = rdr
        .deserialize()
        .filter_map(|t: Result<Emote, _>| t.ok())
        .collect();
    emotes.sort_unstable_by(|a, b| a.name.cmp(&b.name));
    emotes
}

pub fn emote_entries() -> Vec<Entry> {
    emotes().into_iter().map(Entry::from).collect()
}

fn hand_guns() -> Vec<HandGun> {
    let mut rdr = csv::Reader::from_reader(DataType::HandGun.data());
    let mut hand_guns: Vec<HandGun> = rdr
        .deserialize()
        .filter_map(|t: Result<HandGun, _>| t.ok())
        .collect();
    hand_guns.sort_unstable_by(|a, b| a.name.cmp(&b.name));
    hand_guns
}

pub fn hand_gun_entries() -> Vec<Entry> {
    hand_guns().into_iter().map(Entry::from).collect()
}

fn head_armor() -> Vec<HeadArmor> {
    let mut rdr = csv::Reader::from_reader(DataType::HeadArmor.data());
    let mut head_armor: Vec<HeadArmor> = rdr
        .deserialize()
        .filter_map(|t: Result<HeadArmor, _>| t.ok())
        .collect();
    head_armor.sort_unstable_by(|a, b| a.name.cmp(&b.name));
    head_armor
}

pub fn head_armor_entries() -> Vec<Entry> {
    head_armor().into_iter().map(Entry::from).collect()
}

fn leg_armor() -> Vec<LegArmor> {
    let mut rdr = csv::Reader::from_reader(DataType::LegArmor.data());
    let mut leg_armor: Vec<LegArmor> = rdr
        .deserialize()
        .filter_map(|t: Result<LegArmor, _>| t.ok())
        .collect();
    leg_armor.sort_unstable_by(|a, b| a.name.cmp(&b.name));
    leg_armor
}

pub fn leg_armor_entries() -> Vec<Entry> {
    leg_armor().into_iter().map(Entry::from).collect()
}

fn long_guns() -> Vec<LongGun> {
    let mut rdr = csv::Reader::from_reader(DataType::LongGun.data());
    let mut long_guns: Vec<LongGun> = rdr
        .deserialize()
        .filter_map(|t: Result<LongGun, _>| t.ok())
        .collect();
    long_guns.sort_unstable_by(|a, b| a.name.cmp(&b.name));
    long_guns
}

pub fn long_gun_entries() -> Vec<Entry> {
    long_guns().into_iter().map(Entry::from).collect()
}

fn melee_weapons() -> Vec<MeleeWeapon> {
    let mut rdr = csv::Reader::from_reader(DataType::MeleeWeapon.data());
    let mut melee_weapons: Vec<MeleeWeapon> = rdr
        .deserialize()
        .filter_map(|t: Result<MeleeWeapon, _>| t.ok())
        .collect();
    melee_weapons.sort_unstable_by(|a, b| a.name.cmp(&b.name));
    melee_weapons
}

pub fn melee_weapon_entries() -> Vec<Entry> {
    melee_weapons().into_iter().map(Entry::from).collect()
}

fn remnant_traits() -> Vec<Trait> {
    let mut rdr = csv::Reader::from_reader(DataType::Trait.data());
    let mut traits: Vec<Trait> = rdr
        .deserialize()
        .filter_map(|t: Result<Trait, _>| t.ok())
        .collect();
    traits.sort_unstable_by(|a, b| a.name.cmp(&b.name));
    traits
}

pub fn remnant_trait_entries() -> Vec<Entry> {
    remnant_traits().into_iter().map(Entry::from).collect()
}

fn rings() -> Vec<Ring> {
    let mut rdr = csv::Reader::from_reader(DataType::Ring.data());
    let mut rings: Vec<Ring> = rdr
        .deserialize()
        .filter_map(|t: Result<Ring, _>| t.ok())
        .collect();
    rings.sort_unstable_by(|a, b| a.name.cmp(&b.name));
    rings
}

pub fn ring_entries() -> Vec<Entry> {
    rings().into_iter().map(Entry::from).collect()
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
        assert_eq!(NUMBER_OF_AMULETS, amulets().len());
    }

    #[test]
    fn all_amulet_ids_unique() {
        let mut amulets = amulets();
        amulets.dedup_by_key(|x| x.id);
        assert_eq!(NUMBER_OF_AMULETS, amulets.len());
    }

    #[test]
    fn amulets_are_sorted_alphabetically() {
        let amulets = amulets();
        assert_eq!("Amulet of Epicaricacy", amulets[0].name);
        assert_eq!("Amulet of Perseverance", amulets[1].name);
        assert_eq!("Vengeance Idol", amulets[NUMBER_OF_AMULETS - 1].name);
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
    fn armor_sets_are_sorted_alphabetically() {
        let armor_sets = armor_sets();
        assert_eq!("Adventurer Set", armor_sets[0].name);
        assert_eq!("Akari Set", armor_sets[1].name);
        assert_eq!("Void Set", armor_sets[NUMBER_OF_ARMOR_SETS - 1].name);
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
    fn body_armor_is_sorted_alphabetically() {
        let body_armor = body_armor();
        assert_eq!("Adventurer Tunic", body_armor[0].name);
        assert_eq!("Akari Garb", body_armor[1].name);
        assert_eq!("Void Carapace", body_armor[NUMBER_OF_BODY_ARMOR - 1].name);
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
    fn hand_guns_are_sorted_alphabetically() {
        let hand_guns = hand_guns();
        assert_eq!("Curse of the Jungle God", hand_guns[0].name);
        assert_eq!("Defiler", hand_guns[1].name);
        assert_eq!("Submachine Gun", hand_guns[NUMBER_OF_HAND_GUNS - 1].name);
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
    fn head_armor_is_sorted_alphabetically() {
        let head_armor = head_armor();
        assert_eq!("Adventurer Goggles", head_armor[0].name);
        assert_eq!("Akari Mask", head_armor[1].name);
        assert_eq!("Void Skull", head_armor[NUMBER_OF_HEAD_ARMOR - 1].name);
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
    fn emotes_are_sorted_alphabetically() {
        let emotes = emotes();
        assert_eq!("Beckon Emote", emotes[0].name);
        assert_eq!("Cheer Emote", emotes[1].name);
        assert_eq!("Yes Emote", emotes[NUMBER_OF_EMOTES - 1].name);
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
    fn leg_armor_is_sorted_alphabetically() {
        let leg_armor = leg_armor();
        assert_eq!("Adventurer Leggings", leg_armor[0].name);
        assert_eq!("Akari Leggings", leg_armor[1].name);
        assert_eq!("Void Greaves", leg_armor[NUMBER_OF_LEG_ARMOR - 1].name);
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
    fn long_guns_are_sorted_alphabetically() {
        let long_guns = long_guns();
        assert_eq!("Assault Rifle", long_guns[0].name);
        assert_eq!("Beam Rifle", long_guns[1].name);
        assert_eq!("Sporebloom", long_guns[NUMBER_OF_LONG_GUNS - 1].name);
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
    fn melee_weapons_are_sorted_alphabetically() {
        let melee_weapons = melee_weapons();
        assert_eq!("Blade of Adventure", melee_weapons[0].name);
        assert_eq!("Butchers Flail", melee_weapons[1].name);
        assert_eq!(
            "World Breaker",
            melee_weapons[NUMBER_OF_MELEE_WEAPONS - 1].name
        );
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
    fn rings_are_sorted_alphabetically() {
        let rings = rings();
        assert_eq!("Aggressor's Bane", rings[0].name);
        assert_eq!("Akari War Band", rings[1].name);
        assert_eq!("Stone Of Balance", rings[NUMBER_OF_RINGS - 1].name);
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

    #[test]
    fn traits_are_sorted_alphabetically() {
        let traits = remnant_traits();
        assert_eq!("Arcane Strike", traits[0].name);
        assert_eq!("Bark Skin", traits[1].name);
        assert_eq!("World Walker", traits[NUMBER_OF_TRAITS - 1].name);
    }
}
