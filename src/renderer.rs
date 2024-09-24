use axum_template::engine::Engine;
use handlebars::Handlebars;

pub fn create() -> Engine<Handlebars<'static>> {
    let mut hbs = Handlebars::new();

    hbs.register_template_file("index", "templates/index.hbs").unwrap();

    Engine::from(hbs)
}
