use crate::pages::{AuthPage, FeedPage, PostPage};
use dioxus::prelude::*;
#[derive(Routable, Clone, PartialEq)]
pub enum Route {
    #[layout(crate::components::AppLayout)]
    #[route("/")]
    FeedPage {},
    #[route("/auth")]
    AuthPage {},
    #[route("/posts/:id")]
    PostPage { id: i64 },
}
