use serde_json::Value;
use std::sync::OnceLock;
use std::{env, fs};

static LANG_DATA: OnceLock<Value> = OnceLock::new();
static SF2E_OVERRIDES: OnceLock<Value> = OnceLock::new();

fn base_path() -> String {
    env::var("SOURCE_DOWNLOAD_PATH").expect(
        "DOWNLOAD PATH NOT SET.. Aborting. Hint: set SOURCE_DOWNLOAD_PATH environmental variable",
    )
}

pub fn lang_data() -> &'static Value {
    LANG_DATA.get_or_init(|| {
        let base_path = base_path();
        let file_content = fs::read_to_string(format!("{base_path}/static/lang/en.json"))
            .expect("failed to read en.json");
        serde_json::from_str(&file_content).expect("failed to parse en.json")
    })
}

pub fn sf2e_data() -> &'static Value {
    SF2E_OVERRIDES.get_or_init(|| {
        let base_path = base_path();
        let file_content =
            fs::read_to_string(format!("{base_path}/static/lang/sf2e-overrides-en.json"))
                .expect("failed to read sf2e-overrides-en.json");
        serde_json::from_str(&file_content).expect("failed to parse en.json")
    })
}
