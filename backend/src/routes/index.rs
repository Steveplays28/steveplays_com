use std::fs;

use common::data::{art::Art, project::Project};
use glob::glob;
use rocket::get;
use static_init::dynamic;

use crate::cli::arguments::ARGS;

#[dynamic]
static ART_JSON: String = {
    let mut art: Vec<Art> = Vec::new();
    for art_piece in glob(&format!(
        "{}/{}",
        ARGS.backend_resources_path.join("art").to_str().unwrap(),
        "**/*.json"
    ))
    .expect("glob should have a correct pattern")
    {
        art.push(
            serde_json::from_str(
                &fs::read_to_string(
                    art_piece.expect("`art_piece` path should be readable by glob"),
                )
                .unwrap(),
            )
            .expect("serde should be able to parse `art_piece`"),
        );
    }
    art.sort();
    serde_json::to_string::<Vec<Art>>(&art).expect("serde should be able to serialize `art`")
};

#[dynamic]
static PROJECTS_JSON: String = {
    let mut projects: Vec<Project> = Vec::new();
    for project in glob(&format!(
        "{}/{}",
        ARGS.backend_resources_path
            .join("projects")
            .to_str()
            .unwrap(),
        "**/*.json"
    ))
    .expect("glob should have a correct pattern")
    {
        projects.push(
            serde_json::from_str(
                &fs::read_to_string(project.expect("`project` path should be readable by glob"))
                    .unwrap(),
            )
            .expect("serde should be able to parse `project`"),
        );
    }
    projects.sort();
    serde_json::to_string::<Vec<Project>>(&projects)
        .expect("serde should be able to serialize `projects`")
};

#[get("/art")]
pub fn art() -> &'static str {
    &ART_JSON
}

#[get("/projects")]
pub fn projects() -> &'static str {
    &PROJECTS_JSON
}
