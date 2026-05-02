mod app;
mod commands;

fn main() {
    console_error_panic_hook::set_once();
    use leptos::prelude::mount_to_body;
    mount_to_body(app::App);
}
