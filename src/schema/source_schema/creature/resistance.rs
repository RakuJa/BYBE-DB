use serde_json::Value;

#[derive(Debug, Clone)]
pub struct Resistance {
    pub name: String,
    pub value: i64,
    pub double_vs: Vec<String>,
    pub exceptions: Vec<String>,
}

impl Resistance {
    pub fn init_from_json(json: &Value) -> Resistance {
        Self {
            name: json
                .get("type")
                .and_then(|x| x.as_str())
                .map(|x| x.to_string())
                .unwrap(),
            value: json
                .get("value")
                .and_then(|x| x.as_i64())
                .expect("Speed value is NaN"),
            double_vs: json
                .get("doubleVs")
                .and_then(|x| x.as_array())
                .unwrap_or(&vec![])
                .iter()
                .map(|x| {
                    x.as_str()
                        .map(|x| x.to_string())
                        .expect("Double VS element is NOT a string")
                })
                .collect(),
            exceptions: json
                .get("exceptions")
                .and_then(|x| x.as_array())
                .unwrap_or(&vec![])
                .iter()
                .map(|x| {
                    x.as_str()
                        .map(|x| x.to_string())
                        .expect("Exception element is NOT a string")
                })
                .collect(),
        }
    }
}
