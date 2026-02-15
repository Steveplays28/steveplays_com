use cruet::Inflector;
use gloo_net::http::Request;
use strum_macros::EnumString;
use yew::{platform::spawn_local, prelude::*};
use yew_router::prelude::*;

use crate::data::project::Project;

const STEVEPLAYS_AUTHOR_NAME: &str = "Steveplays";

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
    #[at("/art")]
    Art,
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
        Route::Art => html! {
            <Art />
        },
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
                    <Link<Route> to={Route::Art}>
                        <p>{ "Art" }</p>
                    </Link<Route>>
                </div>
                <div class="nav-bar-element">
                    <Link<Route> to={Route::Projects}>
                        <p>{ "Projects" }</p>
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

            <div class="button-grid">
                <a href="/art" rel="noopener noreferrer" class="button">
                    <p>{ "View art" }</p>
                </a>
                <a href="/projects" rel="noopener noreferrer" class="button">
                    <p>{ "View projects" }</p>
                </a>
                <a href="/contact" rel="noopener noreferrer" class="button">
                    <p>{ "Contact" }</p>
                </a>
            </div>
        </>
    }
}

#[function_component(Art)]
pub fn art() -> Html {
    let art_option = use_state(|| None);
    {
        let art = art_option.clone();
        use_effect(move || {
            if art.is_none() {
                spawn_local(async move {
                    let response = Request::get("/index/art").send().await.unwrap();
                    if response.ok() {
                        art.set(Some(Ok(response
                            .json::<Vec<crate::data::art::Art>>()
                            .await
                            .unwrap())));
                    } else {
                        art.set(Some(Err(format!(
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
            <h1 class="header-text">{"Art"}</h1>

            <div class="projects">
            {
                if let Some(Ok(art)) = art_option.as_ref() {
                    art.iter().map(|art| {
                        let style = format!("background-image: url({image});", image = art.image.clone());
                        html! {
                            <div class="project" style={style}>
                                <div class="project-title">
                                    <p>{ format!("{name} ({date})", name = art.name.clone(), date = art.date.clone()) }</p>
                                </div>
                                <div class="project-tags">
                                    if let Some(art_tags) = &art.tags {
                                        for tag in art_tags.iter().map(|tag| {tag.to_sentence_case().to_lowercase()}) {
                                            <p class="project-tag">{ tag }</p>
                                        }
                                    }
                                </div>
                            </div>
                        }
                    }).collect::<Html>()
                } else {
                    html! {
                        <NoResponseText></NoResponseText>
                    }
                }
            }
            </div>
        </>
    }
}

#[function_component(Projects)]
pub fn projects() -> Html {
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
            <h1 class="header-text">{"Projects"}</h1>

            // TODO: Deduplicate the projects
            <div class="projects">
            {
                if let Some(Ok(projects)) = projects_option.as_ref() {
                    projects.iter().filter(|project| {
                        let mut project_is_contribution = false;
                        if let Some(project_contributors) = &project.contributors {
                            for project_contributor in project_contributors {
                                if project_contributor.name.contains(STEVEPLAYS_AUTHOR_NAME) {
                                    project_is_contribution = true;
                                }
                            }
                        }
                        !project_is_contribution
                    }).map(|project| {
                        let main_link = if let Some(project_links) = &project.links {
                            if project_links.website.as_ref().is_some() {
                                project_links.website.as_ref()
                            } else if project_links.repository.as_ref().is_some() {
                                project_links.repository.as_ref()
                            } else {
                                None
                            }
                        } else {
                            None
                        };
                        let style = if let Some(project_images) = &project.images && let Some(project_banner_image) = &project_images.banner { format!("background-image: url({image});", image = project_banner_image.clone()) } else { String::from("") };
                        html! {
                            <a href={main_link.expect("project should have a website or repository link").clone()} target="_blank" rel="noopener noreferrer" key={project.name.clone()} class="project" style={style}>
                                <div class="project-title">
                                    if let Some(project_images) = &project.images && let Some(project_icon_image) = &project_images.icon {
                                        <img src={project_icon_image.clone()} class="project-icon" />
                                    }

                                    <p>{ project.name.clone() }</p>
                                </div>
                                <div class="project-tags">
                                    if let Some(project_authors) = &project.authors && !(project_authors.len() <= 1 && project_authors.first().is_some_and(|project_author| project_author.name == STEVEPLAYS_AUTHOR_NAME)) {
                                        for project_author in project_authors {
                                            <p class="project-tag">{ project_author.name.clone() }</p>
                                        }
                                    }
                                    if let Some(project_contributors) = &project.contributors {
                                        for project_contributor in project_contributors {
                                            <p class="project-tag">{ project_contributor.name.clone() }</p>
                                        }
                                    }
                                </div>
                                <div class="project-tags">
                                    if let Some(project_links) = &project.links && let Some(project_repository_link) = &project_links.repository {
                                        <a href={project_repository_link.clone()} target="_blank" rel="noopener noreferrer" key={project.name.clone()} class="project-tag project-repository-button">
                                            <p>{ "View repository" }</p>
                                            <img src="/resources/icons/outbound_link.svg" class="project-repository-outbound-link-icon"/>
                                        </a>
                                    }
                                    if let Some(project_tags) = &project.tags {
                                        for tag in project_tags.iter().map(|tag| {tag.to_sentence_case().to_lowercase()}) {
                                            <p class="project-tag">{ tag }</p>
                                        }
                                    }
                                </div>
                            </a>
                        }
                    }).collect::<Html>()
                } else {
                    html! {
                        <NoResponseText></NoResponseText>
                    }
                }
            }
            </div>

            <h2 class="header-text">{"Contributions"}</h2>

            <div class="projects">
            {
                if let Some(Ok(projects)) = projects_option.as_ref() {
                    projects.iter().filter(|project| {
                        let mut project_is_contribution = false;
                        if let Some(project_contributors) = &project.contributors {
                            for project_contributor in project_contributors {
                                if project_contributor.name.contains(STEVEPLAYS_AUTHOR_NAME) {
                                    project_is_contribution = true;
                                }
                            }
                        }
                        project_is_contribution
                    }).map(|project| {
                        let main_link = if let Some(project_links) = &project.links {
                            if project_links.website.as_ref().is_some() {
                                project_links.website.as_ref()
                            } else if project_links.repository.as_ref().is_some() {
                                project_links.repository.as_ref()
                            } else {
                                None
                            }
                        } else {
                            None
                        };
                        let style = if let Some(project_images) = &project.images && let Some(project_banner_image) = &project_images.banner { format!("background-image: url({image});", image = project_banner_image.clone()) } else { String::from("") };
                        html! {
                            <a href={main_link.expect("project should have a website or repository link").clone()} target="_blank" rel="noopener noreferrer" key={project.name.clone()} class="project" style={style}>
                                <div class="project-title">
                                    if let Some(project_images) = &project.images && let Some(project_icon_image) = &project_images.icon {
                                        <img src={project_icon_image.clone()} class="project-icon" />
                                    }

                                    <p>{ project.name.clone() }</p>
                                </div>
                                <div class="project-tags">
                                    if let Some(project_authors) = &project.authors && !(project_authors.len() <= 1 && project_authors.first().is_some_and(|project_author| project_author.name == STEVEPLAYS_AUTHOR_NAME)) {
                                        for project_author in project_authors {
                                            <p class="project-tag">{ project_author.name.clone() }</p>
                                        }
                                    }
                                    if let Some(project_contributors) = &project.contributors {
                                        for project_contributor in project_contributors {
                                            <p class="project-tag">{ project_contributor.name.clone() }</p>
                                        }
                                    }
                                </div>
                                <div class="project-tags">
                                    if let Some(project_links) = &project.links && let Some(project_repository_link) = &project_links.repository {
                                        <a href={project_repository_link.clone()} target="_blank" rel="noopener noreferrer" key={project.name.clone()} class="project-tag project-repository-button">
                                            <p>{ "View repository" }</p>
                                            <img src="/resources/icons/outbound_link.svg" class="project-repository-outbound-link-icon"/>
                                        </a>
                                    }
                                    if let Some(project_tags) = &project.tags {
                                        for tag in project_tags.iter().map(|tag| {tag.to_sentence_case().to_lowercase()}) {
                                            <p class="project-tag">{ tag }</p>
                                        }
                                    }
                                </div>
                            </a>
                        }
                    }).collect::<Html>()
                } else {
                    html! {
                        <NoResponseText></NoResponseText>
                    }
                }
            }
            </div>
        </>
    }
}

#[function_component(Contact)]
pub fn contact() -> Html {
    html! {
        <>
            <div class="header-text">
                <h1>{ "Reach out to me" }</h1> <br/>
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

#[function_component(NoResponseText)]
fn no_response_text() -> Html {
    html! {
        <p class="no-response">{ "No response from the server, you may be offline." }</p>
    }
}
