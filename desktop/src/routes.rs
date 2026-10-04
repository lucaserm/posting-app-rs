use crate::pages::{AuthPage, ComposePostPage, FeedPage, PostPage, ProfilePage};
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
    #[route("/posts/new")]
    ComposePostPage {},
    #[route("/profile")]
    ProfilePage {},
}
