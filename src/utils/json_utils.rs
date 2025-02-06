use maplit::hashmap;
use serde_json::Value;
use std::collections::HashMap;

pub fn get_field_from_json(json: &Value, field: &str) -> Value {
    json.get(field)
        .unwrap_or(&Value::String("".to_string()))
        .clone()
}

/// Extract array values from a json with the structure
///
/// `json`: {
///
/// >"random_field": x, "`vec_parent_key`":{
///
/// >>"f1":o1, "f2": o2, "`vec_field_key`": \[obj1, obj2]
///
/// >}
///
/// }
///
/// into \["obj1", "obj2"], extracting the vector and converting its content to string
pub fn extract_vec_of_str_from_json_with_vec_of_jsons(
    json: &Value,
    vec_parent_key: &str,
    vec_field_key: &str,
) -> Vec<String> {
    let vec: Vec<String> = get_field_from_json(json, vec_parent_key)
        .as_array()
        .unwrap_or(&vec![])
        .iter()
        .filter_map(|v| v.get(vec_field_key))
        .filter_map(|v| v.as_str().map(String::from))
        .collect();
    vec
}

pub fn from_json_vec_of_str_to_vec_of_str(json_vec: &[Value]) -> Vec<String> {
    let vec: Vec<String> = json_vec
        .iter()
        .map(|value| value.as_str().map(String::from).unwrap())
        .collect();
    vec
}

pub fn from_json_vec_of_maps_to_map(json: &Value, field: &str) -> Option<HashMap<String, i64>> {
    let mut map = hashmap! {};
    let binding = get_field_from_json(json, field);
    let json_maps = binding.as_array();

    json_maps?;

    for curr_json_map in json_maps.unwrap() {
        map.insert(
            curr_json_map
                .get("type")
                .and_then(|x| x.as_str())
                .map(|x| x.to_string())
                .unwrap(),
            curr_json_map
                .get("value")
                .and_then(|x| x.as_i64())
                .expect("Speed value is NaN"),
        );
    }
    Some(map)
}
