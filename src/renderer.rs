use axum_template::engine::Engine;
use handlebars::Handlebars;

pub fn create() -> Engine<Handlebars<'static>> {
    let mut hbs = Handlebars::new();

    hbs.register_template_file("layout", "views/layouts/default.hbs").unwrap();

    hbs.register_template_file("index", "views/index.hbs").unwrap();
    hbs.register_template_file("about", "views/about.hbs").unwrap();
    hbs.register_template_file("users", "views/user/list.hbs").unwrap();

    hbs.register_template_file("nav", "views/partials/nav.hbs").unwrap();

    Engine::from(hbs)
}
