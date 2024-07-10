use crate::schema::publication_info::PublicationInfo;
use crate::schema::source_schema::description::Description;
use crate::utils::json_utils;
use serde_json::Value;

#[derive(Debug, Clone)]
pub struct Skill {
    pub name: String,
    pub description: Option<String>,
    pub modifier: i64,
    pub proficiency: i64,
    pub publication_info: PublicationInfo,
    pub variant_label: Vec<String>,
}

impl Skill {
    pub fn init_from_json(json: &Value) -> Skill {
        let system_json = json_utils::get_field_from_json(json, "system");
        let description_json = json_utils::get_field_from_json(&system_json, "description");
        let modifier_json = json_utils::get_field_from_json(&system_json, "mod");
        let proficiency_json = json_utils::get_field_from_json(&system_json, "proficient");
        let publication_json = json_utils::get_field_from_json(&system_json, "publication");
        // let rules_json = json_utils::get_field_from_json(&system_json, "rules");
        let variants_json = json_utils::get_field_from_json(&system_json, "variants");
        Skill {
            name: json_utils::get_field_from_json(json, "name")
                .as_str()
                .unwrap()
                .to_string(),
            description: json_utils::get_field_from_json(&description_json, "value")
                .as_str()
                .map(|x| Description::initialize(x).to_string()),
            modifier: json_utils::get_field_from_json(&modifier_json, "value")
                .as_i64()
                .unwrap(),
            proficiency: json_utils::get_field_from_json(&proficiency_json, "value")
                .as_i64()
                .unwrap(),
            publication_info: PublicationInfo::init_from_json(&publication_json),
            variant_label: (0..)
                .map(|i| {
                    let variant_data_json = variants_json.get(i.to_string().as_str());
                    if let Some(curr_skill_variant) = variant_data_json {
                        curr_skill_variant.get("label")
                    } else {
                        None
                    }
                })
                .take_while(|x| x.is_some())
                .map(|x| x.unwrap().as_str().unwrap().to_string())
                .collect(),
        }
    }
}
