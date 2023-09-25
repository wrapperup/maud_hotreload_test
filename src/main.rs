use axum::{Router, routing::get};
use maud::{Markup, html_hotreload, html};
use notify::Watcher;
use std::path::Path;
use tower_http::services::ServeDir;
use tower_livereload::LiveReloadLayer;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let livereload = LiveReloadLayer::new();
    let reloader = livereload.reloader();
    let app = Router::new()
        .route("/hotreload", get(h_maud_control_structures))
        .route("/", get(maud_control_structures))
        .layer(livereload);

    let mut watcher = notify::recommended_watcher(move |_| reloader.reload())?;
    watcher.watch(Path::new("src"), notify::RecursiveMode::Recursive)?;

    axum::Server::bind(&"0.0.0.0:3030".parse()?)
        .serve(app.into_make_service())
        .await?;

    Ok(())
}

// not hotreloadable... :(
fn cool_component(stuff: Markup) -> Markup {
    html! {
        h3 {
            "hi"
        }
        (stuff)
    }
}

#[axum::debug_handler]
async fn maud_control_structures() -> Markup {
    let user = "mike";
    html! {
        h1 {
            "Hello: " (user)
        }
        body {
            "world"
        }
        (cool_component(html! {
            h4 {}
        }))
    }
}

// hotreloadable!
fn h_cool_component(stuff: Markup) -> Markup {
    html_hotreload! {
        h3 {
            "hi"
            (stuff)
        }
    }
}

#[axum::debug_handler]
async fn h_maud_control_structures() -> Markup {
    let user = "mike";
    let poggers = "poggers";
    html_hotreload! {
        h1 {
            "Hello and whats up: " (user)
        }
        h2 {
            "cool beans"
        }
        body {
            "worldaweasiufasjgasdk"
        }
        (h_cool_component(html_hotreload! {
            h4 {
                "hi"
                (user)
            }
        }))
    }
}
