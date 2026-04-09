#![windows_subsystem = "windows"]

slint::include_modules!();

pub mod package;
pub mod pet;
pub mod window;
pub mod app;

fn main() {
    env_logger::init();
    let my_app = app::App::new();
    my_app.run();
}
