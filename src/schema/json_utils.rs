use maplit::hashmap;
use serde_json::Value;
use std::collections::HashMap;

pub(crate) fn get_field_from_json(json: &Value, field: &str) -> Value {
    json.get(field)
        .unwrap_or(&Value::String("".to_string()))
        .clone()
}

pub(crate) fn from_json_vec_of_jsons_convert_to_array_of_str(
    json: &Value,
    field: &str,
    vec_field: &str,
) -> Vec<String> {
    // Converts json with the structure
    // "random_field: x, field:{f1:o1, f2: o2, vec_field:[obj1,obj2]}"  into
    // ["obj1", "obj2"], extracting the vector and converting its content to string
    let vec: Vec<String> = get_field_from_json(json, field)
        .as_array()
        .unwrap_or(&vec![])
        .iter()
        .map(|value| match value.get(vec_field) {
            None => "".to_string(),
            Some(x) => x.as_str().unwrap().to_string(),
        })
        .collect();
    vec
}

pub(crate) fn from_json_vec_of_str_to_vec_of_str(json_vec: &[Value]) -> Vec<String> {
    let vec: Vec<String> = json_vec
        .iter()
        .map(|value| value.as_str().unwrap().to_string().clone())
        .collect();
    vec
}

pub(crate) fn from_json_vec_of_maps_to_map(
    json: &Value,
    field: &str,
) -> Option<HashMap<String, i64>> {
    let mut map = hashmap! {};
    let binding = get_field_from_json(json, field);
    let json_maps = binding.as_array();

    json_maps?;

    for curr_json_map in json_maps.unwrap() {
        map.insert(
            curr_json_map
                .get("type")
                .unwrap()
                .as_str()
                .unwrap()
                .to_string(),
            curr_json_map
                .get("value")
                .unwrap()
                .as_i64()
                .expect("Speed value is NaN"),
        );
    }
    Some(map)
}
