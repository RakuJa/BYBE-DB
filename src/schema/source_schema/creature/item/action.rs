use crate::schema::publication_info::PublicationInfo;
use crate::utils::json_utils;
use serde_json::Value;
#[derive(Debug, Clone)]
pub struct Action {
    pub name: String,
    pub action_type: String,
    pub n_of_actions: Option<i64>,
    pub category: String,
    pub description: String,
    pub publication_info: PublicationInfo,
    pub slug: Option<String>,
    pub traits: ActionTraits,
}

impl Action {
    pub fn init_from_json(json: Value) -> Action {
        let system_json = json_utils::get_field_from_json(&json, "system");
        let publication_json = json_utils::get_field_from_json(&system_json, "publication");
        let action_type_json = json_utils::get_field_from_json(&system_json, "actionType");
        let action_json = json_utils::get_field_from_json(&system_json, "actions");
        let category_json = json_utils::get_field_from_json(&system_json, "category");
        let description_json = json_utils::get_field_from_json(&system_json, "description");
        let slug_json = json_utils::get_field_from_json(&system_json, "slug");
        let traits_json = json_utils::get_field_from_json(&system_json, "traits");
        Action {
            name: json_utils::get_field_from_json(&json, "name")
                .as_str()
                .unwrap()
                .to_string(),
            action_type: json_utils::get_field_from_json(&action_type_json, "value")
                .as_str()
                .unwrap()
                .to_string(),
            n_of_actions: json_utils::get_field_from_json(&action_json, "value")
                .as_i64().and_then(Some),
            category: category_json.as_str().unwrap().to_string(),
            description: json_utils::get_field_from_json(&description_json, "value")
                .as_str()
                .unwrap()
                .to_string(),
            publication_info: PublicationInfo::init_from_json(&publication_json),
            slug: slug_json.as_str().and_then(|x| Some(x.to_string())),
            traits: ActionTraits::init_from_json(&traits_json),
        }
    }
}

#[derive(Debug, Clone)]
pub struct ActionTraits {
    pub rarity: String,
    pub traits: Vec<String>,
}

impl ActionTraits {
    pub fn init_from_json(json: &Value) -> ActionTraits {
        ActionTraits {
            rarity: json_utils::get_field_from_json(json, "rarity")
                .as_str()
                .unwrap()
                .to_string(),
            traits: json_utils::from_json_vec_of_str_to_vec_of_str(
                json_utils::get_field_from_json(json, "value")
                    .as_array()
                    .unwrap(),
            ),
        }
    }
}
