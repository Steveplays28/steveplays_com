use std::fs;
use std::path::PathBuf;
use std::str::FromStr;

use backend::routes::index::{self};
use clap::Parser;
use common::ui::components::{switch, NavBar, Route};
use rocket::fs::relative;
use rocket::fs::FileServer;
use rocket::http::Method;
use rocket::http::Status;
use rocket::response::content::RawHtml;
use rocket::route::{Handler, Outcome};
use rocket::{Data, Request};
use static_init::dynamic;
use yew::prelude::*;
use yew::ServerRenderer;
use yew_router::history::AnyHistory;
use yew_router::history::History;
use yew_router::history::MemoryHistory;
use yew_router::prelude::*;

#[macro_use]
extern crate rocket;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// The path of the backend's resources directory.
    #[arg(short, long, default_value_t = String::from("./resources"))]
    backend_resources_path: String,
    /// The path of the frontend's dist directory.
    #[arg(short, long, default_value_t = String::from("../frontend/dist"))]
    frontend_dist_path: String,
}

#[derive(Properties, PartialEq, Debug)]
struct ServerAppProps {
    path: PathBuf,
}

#[derive(Clone)]
struct CustomHandler();

#[rocket::async_trait]
impl Handler for CustomHandler {
    async fn handle<'r>(&self, req: &'r Request<'_>, data: Data<'r>) -> Outcome<'r> {
        let route = req.segments::<PathBuf>(0..).unwrap();
        let route_result = Route::from_str(route.to_str().unwrap());
        if route_result.is_ok() {
            Outcome::from(
                req,
                render(PathBuf::from_str(route.to_str().unwrap()).unwrap()).await,
            )
        } else {
            Outcome::forward(data, Status::Accepted)
        }
    }
}

impl From<CustomHandler> for Vec<rocket::route::Route> {
    fn from(val: CustomHandler) -> Self {
        vec![
            rocket::route::Route::new(Method::Get, "/", val.clone()),
            rocket::route::Route::new(Method::Get, "/<path..>", val),
        ]
    }
}

#[dynamic]
static ARGS: Args = Args::parse();

#[get("/status")]
fn status() -> &'static str {
    "OK"
}

#[get("/<path..>")]
async fn render(path: PathBuf) -> RawHtml<String> {
    let index_html = fs::read_to_string(
        fs::canonicalize(PathBuf::from(&ARGS.frontend_dist_path))
            .unwrap()
            .join("index.html"),
    )
    .unwrap();
    let server_app_props = ServerAppProps { path };
    let content_html = ServerRenderer::<ServerApp>::with_props(|| server_app_props)
        .render()
        .await;
    RawHtml(index_html.replace("<body>", &format!("<body>{}", content_html)))
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![status])
        .mount("/index", routes![index::projects])
        .mount(
            "/projects",
            FileServer::from(
                fs::canonicalize(PathBuf::from(&ARGS.backend_resources_path))
                    .unwrap()
                    .join("projects"),
            ),
        )
        .mount(
            "/",
            FileServer::from(fs::canonicalize(PathBuf::from(&ARGS.frontend_dist_path)).unwrap())
                .rank(0),
        )
        .mount("/", CustomHandler())
}

#[function_component(ServerApp)]
fn server_app(props: &ServerAppProps) -> Html {
    let history = AnyHistory::from(MemoryHistory::new());
    history.push(format!("/{}", props.path.to_str().unwrap()));

    html! {
        <>
            <Router history={history}>
                <NavBar />

                // Must be child of <Router>
                <Switch<Route> render={switch} />
            </Router>
        </>
    }
}
