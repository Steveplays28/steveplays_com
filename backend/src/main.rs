use std::{env, fs};
use std::path::PathBuf;
use std::process::Command;
use std::str::FromStr;

use backend::cli::arguments::ARGS;
use backend::routes::index::{self};
use common::ui::components::{switch, NavBar, Route};
use rocket::fs::FileServer;
use rocket::http::Method;
use rocket::http::Status;
use rocket::response::content::RawHtml;
use rocket::route::{Handler, Outcome};
use rocket::{Data, Request};
use yew::prelude::*;
use yew::ServerRenderer;
use yew_router::history::AnyHistory;
use yew_router::history::History;
use yew_router::history::MemoryHistory;
use yew_router::prelude::*;

#[macro_use]
extern crate rocket;

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

#[get("/status")]
fn status() -> &'static str {
    "OK"
}

#[get("/<path..>")]
async fn render(path: PathBuf) -> RawHtml<String> {
    let index_html = fs::read_to_string(
        dunce::canonicalize(PathBuf::from(&ARGS.frontend_dist_path))
            .expect("Should be able to access `frontend_dist_path`")
            .join("index.html"),
    )
    .expect("Should be able to read index.html.");
    let server_app_props = ServerAppProps { path };
    let content_html = ServerRenderer::<ServerApp>::with_props(|| server_app_props)
        .render()
        .await;
    RawHtml(index_html.replace("<body>", &format!("<body>{}", content_html)))
}

#[launch]
fn rocket() -> _ {
    // DEBUG
    println!(
        "File tree: {:#?}",
        Command::new("trs")
            .current_dir(env::current_dir().unwrap())
            .arg("-f")
            .output()
            .expect("Failed to execute command")
            .stdout
            .as_slice()
    );

    rocket::build()
        .mount("/", routes![status])
        .mount("/index", routes![index::projects])
        .mount(
            "/projects",
            FileServer::from(
                dunce::canonicalize(PathBuf::from(&ARGS.backend_resources_path))
                    .expect("Should be able to access `backend_resources_path`")
                    .join("projects"),
            ),
        )
        .mount(
            "/",
            FileServer::from(
                dunce::canonicalize(PathBuf::from(&ARGS.frontend_dist_path))
                    .expect("Should be able to access `frontend_dist_path`"),
            )
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
