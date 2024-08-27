use clap::Parser;
use static_init::dynamic;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// The path of the backend's resources directory.
    #[arg(short, long, default_value_t = String::from("./resources"))]
    pub backend_resources_path: String,
    /// The path of the frontend's dist directory.
    #[arg(short, long, default_value_t = String::from("../frontend/dist"))]
    pub frontend_dist_path: String,
}

#[dynamic]
pub static ARGS: Args = Args::parse();
