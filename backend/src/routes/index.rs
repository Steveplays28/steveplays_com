use std::{fs, path::PathBuf};

use common::data::project::Project;
use glob::glob;
use rocket::get;
use static_init::dynamic;

use crate::cli::arguments::ARGS;

#[dynamic]
static PROJECTS_JSON: String = {
    let mut projects: Vec<Project> = Vec::new();
    for project in glob(&format!(
        "{}/{}",
        dunce::canonicalize(PathBuf::from(&ARGS.backend_resources_path))
            .expect("Should be able to access `backend_resources_path`.")
            .join("projects")
            .to_str()
            .unwrap(),
        "**/*.json"
    ))
    .expect("glob should have a correct pattern.")
    {
        projects.push(
            serde_json::from_str(
                &fs::read_to_string(project.expect("`project` path should be readable by glob."))
                    .unwrap(),
            )
            .expect("serde should be able to parse `project`."),
        );
    }

    println!("{}", dunce::canonicalize(PathBuf::from(&ARGS.backend_resources_path))
            .expect("Should be able to access `backend_resources_path`.")
            .join("projects")
            .to_str()
            .unwrap());

    return serde_json::to_string::<Vec<Project>>(&projects)
        .expect("serde should be able to serialize `projects`.");
};

#[get("/projects")]
pub fn projects() -> &'static str {
    &PROJECTS_JSON
}
