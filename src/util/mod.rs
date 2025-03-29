use axum_template::engine::Engine;
use tera::Tera;

pub type AppEngine = Engine<Tera>;

#[derive(Clone)]
pub struct AppState {
    pub engine: AppEngine,
}
