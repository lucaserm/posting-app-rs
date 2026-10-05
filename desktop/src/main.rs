mod api;
mod app;
mod auth;
mod components;
mod pages;
mod platform;
mod routes;
mod types;

fn main() {
    dioxus::launch(app::App);
}
