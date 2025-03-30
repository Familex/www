use std::net::Ipv4Addr;

use axum::{routing::get, Router};
use axum_template::engine::Engine;
use listenfd::ListenFd;
use tera::Tera;
use tokio::net::TcpListener;
use tower_livereload::LiveReloadLayer;
use util::AppState;

mod route;
mod util;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let app = app()?;

    let mut listenfd = ListenFd::from_env();
    // FIXME remove on release
    let listener = match listenfd.take_tcp_listener(0)? {
        // if we are given a tcp listener on listen fd 0, we use that one
        Some(listener) => {
            listener.set_nonblocking(true)?;
            TcpListener::from_std(listener)?
        }
        // otherwise fall back to local listening
        None => TcpListener::bind((Ipv4Addr::LOCALHOST, 3000)).await?,
    };

    println!("listening on {}", listener.local_addr()?);
    axum::serve(listener, app).await?;

    Ok(())
}

fn app() -> anyhow::Result<Router> {
    let tera = Tera::new("template/**/*.jinja")?;
    Ok(Router::new()
        .route("/", get(route::home::index))
        .route("/asset/{*file}", get(route::asset))
        .with_state(AppState {
            engine: Engine::from(tera),
        })
        .layer(LiveReloadLayer::new()))
}
