use std::fs;
use std::path::PathBuf;
use std::str::FromStr;

use backend::cli::arguments::ARGS;
use backend::routes::index::{self};
use common::ui::components::{NavBar, Route, switch};
use normpath::PathExt;
use rocket::fs::FileServer;
use rocket::http::Method;
use rocket::http::Status;
use rocket::response::content::RawHtml;
use rocket::route::{Handler, Outcome};
use rocket::{Data, Request};
use yew::ServerRenderer;
use yew::prelude::*;
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
        let route = req.segments::<PathBuf>(0..).unwrap_or_default();
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
    assert!(fs::exists(&ARGS.frontend_dist_path).is_ok());
    let index_html = fs::read_to_string(
        ARGS.frontend_dist_path
            .normalize()
            .expect("Should be able to normalize `frontend_dist_path`")
            .join("index.html"),
    )
    .expect("Should be able to read index.html");
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
        .mount("/index", routes![index::art, index::projects])
        .mount(
            "/projects",
            FileServer::from(
                ARGS.backend_resources_path
                    .join("projects")
                    .normalize()
                    .expect("Should be able to normalize the backend projects path"),
            ),
        )
        .mount(
            "/",
            FileServer::from(
                ARGS.frontend_dist_path
                    .normalize()
                    .expect("Should be able to normalize `frontend_dist_path`"),
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
