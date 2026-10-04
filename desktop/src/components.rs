use crate::{
    auth::use_auth,
    routes::Route,
    types::{Comment, Post},
};
use dioxus::prelude::*;

#[component]
pub fn AppLayout() -> Element {
    let auth = use_auth();
    rsx! { main { class: "app-shell", div { class: "glow glow-one" } div { class: "glow glow-two" }
    div { class: "container", header { class: "topbar",
        Link { class: "brand", to: Route::FeedPage {}, div { class: "brand-mark", "R" } div { p { class: "brand-name", "Rustboard" } p { class: "brand-caption", "Community notes, built in Rust" } } }
            nav { class: "nav-links", Link { to: Route::FeedPage {}, "Feed" }
                if auth().is_some() { Link { to: Route::ComposePostPage {}, "Write" } Link { to: Route::ProfilePage {}, "Profile" } } else { Link { to: Route::AuthPage {}, "Sign in" } }
            }
    } Outlet::<Route> {} } } }
}

#[component]
pub fn PostCard(post: Post) -> Element {
    rsx! { article { class: "post-card",
        div { class: "post-meta", span { class: "post-id", "Post #{post.id}" } span { class: "meta-dot" } span { "{post.comment_count} replies" } }
        h2 { class: "post-title", "{post.title}" } p { class: "post-content", "{post.content}" }
        Link { class: "discussion-button", to: Route::PostPage { id: post.id }, "Open discussion " span { "→" } }
    } }
}
#[component]
pub fn EmptyState(title: String, detail: String) -> Element {
    rsx! { div { class: "empty-state", div { class: "empty-icon", "✦" } h3 { "{title}" } p { "{detail}" } } }
}
#[component]
pub fn CommentList(comments: Vec<Comment>) -> Element {
    rsx! { div { class: "comment-list",
        if comments.is_empty() { p { class: "muted", "No replies yet. Start the conversation." } }
        for comment in comments { article { key: "{comment.id}", class: "comment-item",
            div { class: "comment-avatar", "{comment.user_id}" }
            div { p { "{comment.content}" } small { "Community member · Reply #{comment.id}" } }
        }}
    } }
}
