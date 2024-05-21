use std::fs;

pub fn get_json_paths(source_path: &str) -> Vec<String> {
    let mut json_list = vec![];
    for curr_manual in get_manuals_paths(source_path) {
        let paths = fs::read_dir(curr_manual.clone())
            .unwrap_or_else(|_| panic!("Could not open manual named: {}", curr_manual));
        for path in paths {
            match path {
                Ok(cr_json_path) => {
                    let x = cr_json_path.path().to_str().unwrap_or_default().to_string();
                    if !x.is_empty() && x.ends_with(".json") {
                        json_list.push(x)
                    }
                }
                Err(err) => println!("Skipping path with error: {}", err),
            }
        }
    }
    json_list
}

fn get_manuals_paths(source_path: &str) -> Vec<String> {
    let mut bestiary_folders = vec![];
    let manuals = fs::read_dir(format!("{}/packs", source_path))
        .expect("No valid packs folder found in source dataset");
    for path in manuals {
        match path {
            Ok(dir_path) => {
                let path = dir_path.path().to_str().unwrap_or_default().to_string();
                bestiary_folders.push(path);
            }
            Err(err) => println!("Skipping path with error: {}", err),
        };
    }
    bestiary_folders
}
