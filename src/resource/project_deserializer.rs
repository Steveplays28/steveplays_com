use serde::Deserialize;
use serde_json;
use std::fs;
use std::path::PathBuf;

const PROJECTS_PATH: &str = "/resources/projects";
const MAX_PROJECT_SEARCH_DEPTH: u32 = 7;

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Project {
    pub name: String,
    pub link: String,
    pub image: String,
}

pub fn deserialize_projects() -> Vec<Project> {
    deserialize_projects_recursively(0, &PathBuf::from(PROJECTS_PATH))
}

fn deserialize_projects_recursively(_current_depth: u32, directory_path: &PathBuf) -> Vec<Project> {
    if _current_depth >= MAX_PROJECT_SEARCH_DEPTH {
        return Vec::new();
    }

    let mut projects: Vec<Project> = Vec::new();
    for entry in fs::read_dir(directory_path).expect("Couldn't read projects directory.") {
        if entry.is_err() {
            println!("Couldn't read directory: {}", directory_path.display());
            continue;
        }

        let entry = entry.unwrap();
        let entry_path = entry.path();
        if entry_path.is_dir() {
            projects.append(&mut deserialize_projects_recursively(
                _current_depth + 1,
                &entry_path,
            ));
        } else {
            let file_data = fs::read_to_string(&entry_path);
            if file_data.is_err() {
                println!("Couldn't read file: {}.", &entry_path.display());
                continue;
            }

            let file_data_json = serde_json::from_str(&fs::read_to_string(&entry_path).unwrap());
            if file_data_json.is_err() {
                println!("Couldn't parse file into JSON: {}.", &entry_path.display());
                continue;
            }

            projects.push(file_data_json.unwrap());
        }
    }

    projects
}
