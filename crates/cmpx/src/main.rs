use gpui::Application;

mod app;
mod ui;

fn main() {
    Application::new()
        .run(app::launch);
}
