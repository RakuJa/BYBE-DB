use crate::schema::source_schema::creature::item::spell::Spell;
use crate::schema::source_schema::creature::item::weapon::Weapon;
use serde_json::Value;

#[derive(Debug)]
pub struct RawItems {
    pub spell_casting: Option<String>,
    pub spell_list: Vec<Spell>,
    pub weapon_list: Vec<Weapon>,
}

impl RawItems {
    pub fn init_from_json(json: Value) -> RawItems {
        let json_vec = json
            .as_array()
            .expect("Items entry is not formatted as a vector, Abort.");
        let mut spell_casting_entry = None;
        let mut spell_list_entry = Vec::new();
        let mut weapon_list_entry = Vec::new();
        for el in json_vec {
            let curr_el_type = el
                .get("type")
                .expect("Field type in item not formatted as expected");
            let curr_type = curr_el_type.as_str().unwrap().to_string();
            match curr_type.to_ascii_lowercase().as_str() {
                "spellcastingentry" => {
                    spell_casting_entry = el.get("name").map(|x| x.as_str().unwrap().to_string())
                }
                "spell" => {
                    spell_list_entry.push(Spell::init_from_json(el.clone()));
                }
                "melee" | "ranged" => {
                    weapon_list_entry.push(Weapon::init_from_json(el.clone()));
                }
                // there are other options
                _ => {
                    // do nothing
                }
            }
        }
        RawItems {
            spell_casting: spell_casting_entry,
            spell_list: spell_list_entry,
            weapon_list: weapon_list_entry,
        }
    }
}
