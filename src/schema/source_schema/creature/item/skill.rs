use crate::schema::publication_info::{PublicationInfo, PublicationParsingError};
use crate::schema::source_schema::description::Description;
use crate::utils::json_utils;
use serde_json::Value;
use thiserror::Error;

#[derive(Debug, Clone)]
pub struct Skill {
    pub name: String,
    pub description: Option<String>,
    pub modifier: i64,
    pub proficiency: i64,
    pub publication_info: PublicationInfo,
    pub variant_label: Vec<String>,
}

#[derive(Debug, Error)]
pub enum SkillParsingError {
    #[error("Name field could not be parsed")]
    NameParsing,
    #[error("Modifier field could not be parsed")]
    ModifierParsing,
    #[error("Proficiency field could not be parsed")]
    ProficiencyParsing,
    #[error("Skill variant field could not be parsed")]
    SkillVariantParsing,
    #[error("Publication could not be parsed")]
    Publication(#[from] PublicationParsingError),
}

impl TryFrom<&Value> for Skill {
    type Error = SkillParsingError;
    fn try_from(json: &Value) -> Result<Self, Self::Error> {
        let system_json = json_utils::get_field_from_json(json, "system");
        let description_json = json_utils::get_field_from_json(&system_json, "description");
        let modifier_json = json_utils::get_field_from_json(&system_json, "mod");
        let proficiency_json = json_utils::get_field_from_json(&system_json, "proficient");
        let publication_json = json_utils::get_field_from_json(&system_json, "publication");
        // let rules_json = json_utils::get_field_from_json(&system_json, "rules");
        let variants_json = json_utils::get_field_from_json(&system_json, "variants");
        Ok(Skill {
            name: json_utils::get_field_from_json(json, "name")
                .as_str()
                .map(String::from)
                .ok_or(SkillParsingError::NameParsing)?,
            description: json_utils::get_field_from_json(&description_json, "value")
                .as_str()
                .map(|x| Description::from(x).to_string()),
            modifier: json_utils::get_field_from_json(&modifier_json, "value")
                .as_i64()
                .ok_or(SkillParsingError::ModifierParsing)?,
            proficiency: json_utils::get_field_from_json(&proficiency_json, "value")
                .as_i64()
                .ok_or(SkillParsingError::ProficiencyParsing)?,
            publication_info: PublicationInfo::try_from(&publication_json)?,
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
                .map(|x| {
                    x.and_then(|x| x.as_str())
                        .map(String::from)
                        .ok_or(SkillParsingError::SkillVariantParsing)
                })
                .collect::<Result<Vec<_>, _>>()?,
        })
    }
}
