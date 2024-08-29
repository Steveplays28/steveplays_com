use std::path::PathBuf;

use clap::{Parser, ValueHint};
use static_init::dynamic;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// The path of the backend's resources directory.
    #[arg(short, long, value_hint = ValueHint::DirPath, default_value = "backend/resources")]
    pub backend_resources_path: PathBuf,
    /// The path of the frontend's dist directory.
    #[arg(short, long, value_hint = ValueHint::DirPath, default_value = "frontend/dist")]
    pub frontend_dist_path: PathBuf,
}

#[dynamic]
pub static ARGS: Args = Args::parse();
