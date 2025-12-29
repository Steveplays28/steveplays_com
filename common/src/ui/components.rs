use gloo_net::http::Request;
use strum_macros::EnumString;
use yew::{platform::spawn_local, prelude::*};
use yew_router::prelude::*;

use crate::data::project::Project;

#[derive(Debug, Clone, Copy, PartialEq, Routable, EnumString)]
pub enum Route {
    #[strum(serialize = "", ascii_case_insensitive)]
    #[at("/")]
    Home,
    #[strum(ascii_case_insensitive)]
    #[not_found]
    #[at("/404")]
    NotFound,
    #[strum(ascii_case_insensitive)]
    #[at("/projects")]
    Projects,
    #[strum(ascii_case_insensitive)]
    #[at("/contact")]
    Contact,
}

pub fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! {
            <Home />
        },
        Route::NotFound => html! { <h1>{ "404" }</h1> },
        Route::Projects => html! {
            <Projects />
        },
        Route::Contact => html! {
            <Contact />
        },
    }
}

#[function_component(NavBar)]
pub fn nav_bar() -> Html {
    html! {
        <>
            <div class="nav-bar">
                <div class="nav-bar-element">
                    <Link<Route> to={Route::Home}>
                        <p>{ "Home" }</p>
                    </Link<Route>>
                </div>
                <div class="nav-bar-element">
                    <Link<Route> to={Route::Contact}>
                        <p>{ "Contact" }</p>
                    </Link<Route>>
                </div>

                <div class="nav-bar-element socials">
                    <a href="https://github.com/Steveplays28" target="_blank" rel="noopener noreferrer">
                        <img src="/resources/logos/github-mark-white.svg" />
                    </a>
                    <a href="https://discord.gg/KbWxgGg" target="_blank" rel="noopener noreferrer">
                        <img src="/resources/logos/discord-mark-white.svg" />
                    </a>
                    <a href="https://www.youtube.com/steveplays28" target="_blank" rel="noopener noreferrer">
                        <img src="/resources/logos/yt_icon_mono_dark.png" />
                    </a>
                    <a href="https://www.patreon.com/steveplays28" target="_blank" rel="noopener noreferrer">
                        <img src="/resources/logos/Digital-Patreon-Logo_White.png" />
                    </a>
                    <a href="https://ko-fi.com/steveplays" target="_blank" rel="noopener noreferrer">
                        <img src="/resources/logos/kofi_s_logo_nolabel.svg" />
                    </a>
                </div>
            </div>
        </>
    }
}

#[function_component(Home)]
fn home() -> Html {
    let projects_option = use_state(|| None);
    {
        let projects = projects_option.clone();
        use_effect(move || {
            if projects.is_none() {
                spawn_local(async move {
                    let response = Request::get("/index/projects").send().await.unwrap();
                    if response.ok() {
                        projects.set(Some(Ok(response.json::<Vec<Project>>().await.unwrap())));
                    } else {
                        projects.set(Some(Err(format!(
                            "Error fetching data {} ({})",
                            response.status(),
                            response.status_text()
                        ))));
                    }
                });
            }

            || {}
        });
    }
    html! {
        <>
            <div class="bio-container">
                <a href="https://github.com/Steveplays28" target="_blank" rel="noopener noreferrer" class="profile-picture-container">
                    <img src="/resources/logos/steve_profile_picture.png" alt="Profile picture" class="profile-picture" />
                    <img src="/resources/logos/github-mark-white-background.svg" alt="GitHub out link" class="github" />
                </a>

                <p class="bio">
                    { "Hi, I'm Steve!" } <br/>
                    { "I like making games!" } <br/><br/>
                    { "I'm currently maintaining libraries, Minecraft mods, a Factorio mod, and a Satisfactory mod. I've also contributed to other Minecraft mods, such as " } <a href="https://modrinth.com/mod/forgero" target="_blank" rel="noopener noreferrer"> { "Forgero" } </a> { " and "} <a href="https://modrinth.com/mod/distanthorizons" target="_blank" rel="noopener noreferrer"> { "Distant Horizons" } </a> { "." }
                </p>
            </div>

            <div class="projects">
            {
                if let Some(Ok(projects)) = projects_option.as_ref() {
                    projects.iter().map(|project| {
                        let style = format!("background-image: url({image});", image = project.image.clone());
                        html! {
                            <a href={project.link.clone()} target="_blank" rel="noopener noreferrer" key={project.name.clone()} class="project" style={style}>
                                <p class="project-title">{ project.name.clone() }</p>
                            </a>
                        }
                    }).collect::<Html>()
                } else {
                    html ! {
                        <p class="no-response">{ "No response from the server, you may be offline." }</p>
                    }
                }
            }
            </div>
        </>
    }
}

#[function_component(Projects)]
pub fn projects() -> Html {
    html! {
        <>
            <h1 class="title">{"Projects"}</h1>
        </>
    }
}

#[function_component(Contact)]
pub fn contact() -> Html {
    html! {
        <>
            <div class="contact-text">
                <p class="title">{ "Reach out to me" }</p> <br/>
                <p class="subtitle">{ "I respond most quickly on Discord, GitHub and Patreon, but I try my best to respond on all platforms." }</p>
            </div>

            <div class="socials contact-socials">
                <a href="https://www.patreon.com/steveplays28" target="_blank" rel="noopener noreferrer">
                    <img src="/resources/logos/Digital-Patreon-Logo_White.png" />
                </a>
                <a href="https://ko-fi.com/steveplays" target="_blank" rel="noopener noreferrer">
                    <img src="/resources/logos/kofi_s_logo_nolabel.svg" />
                </a>
                <div class="gap"></div>

                <a href="https://github.com/Steveplays28" target="_blank" rel="noopener noreferrer">
                    <img src="/resources/logos/github-mark-white.svg" />
                </a>
                <a href="https://gitlab.com/steveplays" target="_blank" rel="noopener noreferrer">
                    <img src="/resources/logos/gitlab-logo-700.svg" />
                </a>
                <div class="gap"></div>

                <a href="https://steveplays.itch.io" target="_blank" rel="noopener noreferrer">
                    <img src="/resources/logos/itchio-logo-textless-white.svg" />
                </a>
                <a href="https://modrinth.com/user/Steveplays" target="_blank" rel="noopener noreferrer">
                    <img src="/resources/logos/modrinth-logo-white.svg" />
                </a>
                <a href="https://www.curseforge.com/members/steveplays28" target="_blank" rel="noopener noreferrer">
                    <img src="/resources/logos/curseforge-logo-white.svg" />
                </a>
                <div class="gap"></div>

                <a href="https://www.youtube.com/steveplays28" target="_blank" rel="noopener noreferrer">
                    <img src="/resources/logos/yt_icon_mono_dark.png" />
                </a>
                <a href="https://twitch.tv/steveplays28" target="_blank" rel="noopener noreferrer">
                    <img src="/resources/logos/TwitchGlitchWhite.svg" />
                </a>
                <div class="gap"></div>

                <a href="https://discord.gg/KbWxgGg" target="_blank" rel="noopener noreferrer">
                    <img src="/resources/logos/discord-mark-white.svg" />
                </a>
                <a href="https://rvlt.gg/gYXfebk5" target="_blank" rel="noopener noreferrer">
                    <img src="/resources/logos/revolt_chat_logo_white.png" />
                </a>
                <a href="https://mastodon.gamedev.place/@steveplays" target="_blank" rel="noopener noreferrer">
                    <img src="/resources/logos/mastodon-logo-white.svg" />
                </a>
                <a href="https://twitter.com/steveplays28" target="_blank" rel="noopener noreferrer">
                    <img src="/resources/logos/twitter-logo-white.svg" />
                </a>
            </div>
        </>
    }
}
