use crate::schema::publication_info::PublicationInfo;
use crate::utils::json_utils;
use serde_json::Value;

#[derive(Debug)]
pub struct RawDetails {
    pub languages_details: String,
    pub languages: Vec<String>,
    pub level: i64, //i8,
    pub publication_info: PublicationInfo,
}

impl RawDetails {
    pub fn init_from_json(json: Value) -> RawDetails {
        let languages_json = &json_utils::get_field_from_json(&json, "languages");
        let publication_json = &json_utils::get_field_from_json(&json, "publication");
        RawDetails {
            level: json_utils::get_field_from_json(&json, "level")
                .get("value")
                .unwrap()
                .as_i64()
                .expect("Level field NaN"),
            languages_details: json_utils::get_field_from_json(languages_json, "details")
                .as_str()
                .unwrap()
                .to_string(),
            languages: json_utils::from_json_vec_of_str_to_vec_of_str(
                json_utils::get_field_from_json(languages_json, "value")
                    .as_array()
                    .unwrap(),
            ),
            publication_info: PublicationInfo::init_from_json(publication_json),
        }
    }
}
