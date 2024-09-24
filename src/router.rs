use axum::routing::get;
use axum::response::IntoResponse;

use axum_template::RenderHtml;
use axum_template::engine::Engine;

use handlebars::Handlebars;

use serde::Serialize;

type AppEngine = Engine<Handlebars<'static>>;

#[derive(Serialize)]
struct Person {
    name: String
}

pub fn create(engine: AppEngine) -> axum::Router {
    axum::Router::new()
        .route("/", get(index))
        .with_state(engine)
}

async fn index(engine: AppEngine) -> impl IntoResponse {
    let key = "index";
    let data = Person { name: "Robby".to_string() };
    RenderHtml(key, engine, data)
}
