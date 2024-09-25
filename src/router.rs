use axum::routing::get;
use axum::response::IntoResponse;

use axum_template::RenderHtml;
use axum_template::engine::Engine;

use handlebars::Handlebars;

use serde::Serialize;
use serde_json::json;

type AppEngine = Engine<Handlebars<'static>>;

#[derive(Serialize)]
struct Person {
    name: &'static str
}

pub fn create(engine: AppEngine) -> axum::Router {
    axum::Router::new()
        .route("/", get(index))
        .route("/about", get(about))
        .route("/users", get(users))
        .with_state(engine)
}

async fn index(engine: AppEngine) -> impl IntoResponse {
    let key = "index";
    let data = json!({"message": "Hello, World!"});
    RenderHtml(key, engine, data)
}

async fn about(engine: AppEngine) -> impl IntoResponse {
    let key = "about";
    let data = json!("{}");
    RenderHtml(key, engine, data)
}

async fn users(engine: AppEngine) -> impl IntoResponse {
    let key = "users";

    let data = json!({"users": vec![
        Person {name: "Alice"},
        Person {name: "Bob"},
        Person {name: "Eve"},
    ]});

    RenderHtml(key, engine, data)
}
