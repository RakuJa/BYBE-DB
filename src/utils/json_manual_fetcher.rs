use log::info;
use std::fs;

pub fn get_json_paths(source_path: &str) -> Vec<String> {
    get_manuals_paths(format!("{source_path}/packs"))
}

fn get_manuals_paths(source_path: String) -> Vec<String> {
    let mut bestiary_folders = vec![];
    let manuals_source_dir =
        fs::read_dir(source_path).expect("No valid bestiary folder found in source dataset");
    for path in manuals_source_dir {
        match path {
            Ok(dir_path) => {
                if dir_path.path().is_dir() {
                    bestiary_folders.extend(get_manuals_paths(
                        dir_path.path().to_str().unwrap().to_string(),
                    ));
                } else if dir_path.path().is_file() {
                    let path = dir_path.path().to_str().unwrap().to_string();
                    if !path.is_empty()
                        && path.ends_with(".json")
                        && !path.ends_with("_folders.json")
                    {
                        bestiary_folders.push(path);
                    }
                }
            }
            Err(err) => info!("Skipping path with error: {err}"),
        };
    }
    bestiary_folders
}
