use crate::{
    api,
    auth::use_auth,
    components::{CommentList, EmptyState, PostCard},
    routes::Route,
    types::LoginRequest,
};
use dioxus::prelude::*;

#[component]
pub fn FeedPage() -> Element {
    let mut sort = use_signal(|| "recent".to_string());
    let posts = use_resource(move || {
        let sort = sort();
        async move { api::posts(&sort).await }
    });
    rsx! { section { class: "page feed-page",
        div { class: "hero", p { class: "eyebrow", "Community feed" } h1 { "A quieter place to share what you’re learning." } p { class: "subtitle", "Read notes from the community, then take the discussion deeper on each post." } }
        div { class: "feed-heading", div { h2 { "Latest posts" } p { "Choose how you want to explore the conversation." } }
            div { class: "sort-control",
                button { class: if sort() == "recent" { "sort-button active" } else { "sort-button" }, onclick: move |_| sort.set("recent".into()), "Most recent" }
                button { class: if sort() == "discussed" { "sort-button active" } else { "sort-button" }, onclick: move |_| sort.set("discussed".into()), "Most discussed" }
            }
        }
        div { class: "feed-grid", match &*posts.read() {
            Some(Ok(items)) if items.is_empty() => rsx! { EmptyState { title: "No posts yet", detail: "The first thoughtful note belongs here." } },
            Some(Ok(items)) => rsx! { for post in items { PostCard { post: post.clone() } } },
            Some(Err(_)) => rsx! { EmptyState { title: "Could not reach the feed", detail: "Make sure the Rust API is running on port 3000, then refresh the app." } },
            None => rsx! { div { class: "loading-card", span { class: "loading-dot" } "Loading the latest posts…" } },
        }}
    } }
}

#[component]
pub fn AuthPage() -> Element {
    let mut auth = use_auth();
    let mut email = use_signal(String::new);
    let mut password = use_signal(String::new);
    let mut feedback = use_signal(String::new);
    let navigator = use_navigator();
    rsx! { section { class: "page auth-page", div { class: "auth-card", p { class: "eyebrow", "Your account" }
        if auth().is_some() {
            h1 { "You are signed in." } p { class: "subtitle", "Your session is remembered on this device." }
            button { class: "button button-secondary", onclick: move |_| auth.set(None), "Sign out" }
        } else {
            h1 { "Welcome back." } p { class: "subtitle", "Sign in to contribute to a post's discussion." }
            form { class: "auth-form", onsubmit: move |event| async move {
                event.prevent_default(); feedback.set("Signing in…".into());
                match api::login(&LoginRequest { email: email(), password: password() }).await {
                    Ok(login) => { auth.set(Some(login.access_token)); password.set(String::new()); navigator.push(Route::FeedPage {}); }
                    Err(_) => feedback.set("We could not sign you in. Check your details and try again.".into()),
                }
            },
                label { "Email address" input { class: "field", r#type: "email", value: "{email}", oninput: move |e| email.set(e.value()) } }
                label { "Password" input { class: "field", r#type: "password", value: "{password}", oninput: move |e| password.set(e.value()) } }
                button { class: "button button-primary", r#type: "submit", "Sign in" }
                if !feedback().is_empty() { p { class: "feedback", "{feedback}" } }
            }
        }
    } } }
}

#[component]
pub fn PostPage(id: i64) -> Element {
    let auth = use_auth();
    let post = use_resource(move || async move { api::post(id).await });
    let mut reload = use_signal(|| 0_u64);
    let comments = use_resource(move || {
        let _version = reload();
        async move { api::comments(id).await }
    });
    let mut content = use_signal(String::new);
    let mut feedback = use_signal(String::new);
    rsx! { section { class: "page post-page", Link { class: "back-link", to: Route::FeedPage {}, "← Back to feed" }
        match &*post.read() {
            Some(Ok(post)) => rsx! { article { class: "post-detail", div { class: "post-meta", span { class: "post-id", "Post #{post.id}" } span { class: "meta-dot" } span { "{post.comment_count} replies" } } h1 { "{post.title}" } p { class: "post-content", "{post.content}" } }},
            Some(Err(_)) => rsx! { EmptyState { title: "Post unavailable", detail: "It may have been removed or the API is not available." } },
            None => rsx! { div { class: "loading-card", "Loading post…" } },
        }
        section { class: "discussion-section", div { class: "comments-heading", div { h2 { "Discussion" } p { "Add something useful to the thread." } } }
            match &*comments.read() { Some(Ok(items)) => rsx! { CommentList { comments: items.clone() } }, Some(Err(_)) => rsx! { p { class: "muted", "Comments could not be loaded." } }, None => rsx! { p { class: "muted", "Loading discussion…" } } }
            if let Some(token) = auth() {
                form { class: "comment-form", onsubmit: move |event| {
                    let token = token.clone();
                    async move {
                    event.prevent_default(); let text = content().trim().to_owned();
                    if text.is_empty() { feedback.set("Write a comment before posting.".into()); return; }
                    match api::create_comment(id, &token, text).await {
                        Ok(_) => { content.set(String::new()); feedback.set("Your reply has been added.".into()); reload.set(reload() + 1); }
                        Err(_) => feedback.set("Your reply could not be added.".into()),
                    }
                }},
                    textarea { class: "field comment-input", value: "{content}", placeholder: "Share a considered response…", oninput: move |e| content.set(e.value()) }
                    div { class: "comment-footer", if !feedback().is_empty() { p { class: "feedback", "{feedback}" } } button { class: "button button-primary", r#type: "submit", "Post reply" } }
                }
            } else { div { class: "signin-hint", "Want to reply? " Link { to: Route::AuthPage {}, "Sign in to join the discussion." } } }
        }
    } }
}
