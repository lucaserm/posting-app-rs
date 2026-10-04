use crate::{
    api,
    auth::use_auth,
    components::{EmptyState, PostCard},
    routes::Route,
    types::LoginRequest,
};
use dioxus::prelude::*;

#[component]
pub fn FeedPage() -> Element {
    let mut sort = use_signal(|| "recent".to_string());
    let mut search = use_signal(String::new);
    let mut offset = use_signal(|| 0_i64);
    let posts = use_resource(move || {
        let sort = sort();
        let search = search();
        let offset = offset();
        async move { api::posts(&sort, &search, offset).await }
    });
    rsx! { section { class: "page feed-page",
        div { class: "hero", p { class: "eyebrow", "Community feed" } h1 { "A quieter place to share what you’re learning." } p { class: "subtitle", "Read notes from the community, then take the discussion deeper on each post." } }
        div { class: "feed-heading", div { h2 { "Latest posts" } p { "Choose how you want to explore the conversation." } }
            div { class: "sort-control",
                button { class: if sort() == "recent" { "sort-button active" } else { "sort-button" }, onclick: move |_| sort.set("recent".into()), "Most recent" }
                button { class: if sort() == "discussed" { "sort-button active" } else { "sort-button" }, onclick: move |_| sort.set("discussed".into()), "Most discussed" }
            }
        }
        div { class: "feed-tools",
            input { class: "field search-field", placeholder: "Search posts…", value: "{search}", oninput: move |e| { search.set(e.value()); offset.set(0); } }
            Link { class: "button button-primary", to: Route::ComposePostPage {}, "Write a post" }
        }
        div { class: "feed-grid", match &*posts.read() {
            Some(Ok(items)) if items.is_empty() => rsx! { EmptyState { title: "No posts yet", detail: "The first thoughtful note belongs here." } },
            Some(Ok(items)) => rsx! { for post in items { PostCard { post: post.clone() } } },
            Some(Err(_)) => rsx! { EmptyState { title: "Could not reach the feed", detail: "Make sure the Rust API is running on port 3000, then refresh the app." } },
            None => rsx! { div { class: "loading-card", span { class: "loading-dot" } "Loading the latest posts…" } },
        }}
        div { class: "pagination",
            button { class: "button button-secondary", disabled: offset() == 0, onclick: move |_| offset.set((offset() - 12).max(0)), "Previous" }
            span { "Page {offset() / 12 + 1}" }
            button { class: "button button-secondary", onclick: move |_| offset.set(offset() + 12), "Next" }
        }
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
    let mut editing = use_signal(|| false);
    let mut edit_title = use_signal(String::new);
    let mut edit_content = use_signal(String::new);
    let navigator = use_navigator();
    rsx! { section { class: "page post-page", Link { class: "back-link", to: Route::FeedPage {}, "← Back to feed" }
        match &*post.read() {
            Some(Ok(post)) => { let post = post.clone(); rsx! { article { class: "post-detail", div { class: "post-meta", span { class: "post-id", "Post #{post.id}" } span { class: "meta-dot" } span { "{post.comment_count} replies" } }
                if editing() {
                    input { class: "field", value: "{edit_title}", oninput: move |e| edit_title.set(e.value()) }
                    textarea { class: "field comment-input", value: "{edit_content}", oninput: move |e| edit_content.set(e.value()) }
                    if let Some(token) = auth() { button { class: "button button-primary", onclick: move |_| { let token = token.clone(); async move { let _ = api::update_post(id, &token, crate::types::PostInput { title: edit_title(), content: edit_content() }).await; editing.set(false); } }, "Save changes" } }
                } else { h1 { "{post.title}" } p { class: "post-content", "{post.content}" }
                    if let Some(token) = auth() { div { class: "owner-actions",
                        button { class: "button button-secondary", onclick: move |_| { edit_title.set(post.title.clone()); edit_content.set(post.content.clone()); editing.set(true); }, "Edit post" }
                        button { class: "button button-danger", onclick: move |_| { let token = token.clone(); async move { if api::delete_post(id, &token).await.is_ok() { navigator.push(Route::FeedPage {}); } } }, "Delete post" }
                    }}
                }
            }} },
            Some(Err(_)) => rsx! { EmptyState { title: "Post unavailable", detail: "It may have been removed or the API is not available." } },
            None => rsx! { div { class: "loading-card", "Loading post…" } },
        }
        section { class: "discussion-section", div { class: "comments-heading", div { h2 { "Discussion" } p { "Add something useful to the thread." } } }
            match &*comments.read() { Some(Ok(items)) => rsx! { div { class: "comment-list", for comment in items { EditableComment { comment: comment.clone(), post_id: id, reload } } } }, Some(Err(_)) => rsx! { p { class: "muted", "Comments could not be loaded." } }, None => rsx! { p { class: "muted", "Loading discussion…" } } }
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

#[component]
fn EditableComment(
    comment: crate::types::Comment,
    post_id: i64,
    mut reload: Signal<u64>,
) -> Element {
    let auth = use_auth();
    let mut editing = use_signal(|| false);
    let mut draft = use_signal(|| comment.content.clone());
    rsx! { article { class: "comment-item", div { class: "comment-avatar", "{comment.user_id}" }
        div { class: "comment-body",
            if editing() { textarea { class: "field comment-input", value: "{draft}", oninput: move |e| draft.set(e.value()) }
                if let Some(token) = auth() { button { class: "text-action", onclick: move |_| { let token = token.clone(); async move { if api::update_comment(post_id, comment.id, &token, draft()).await.is_ok() { editing.set(false); reload.set(reload() + 1); } } }, "Save" } }
            } else { p { "{comment.content}" } small { "Community member · Reply #{comment.id}" }
                if let Some(token) = auth() { div { class: "comment-actions",
                    button { class: "text-action", onclick: move |_| editing.set(true), "Edit" }
                    button { class: "text-action danger-text", onclick: move |_| { let token = token.clone(); async move { if api::delete_comment(post_id, comment.id, &token).await.is_ok() { reload.set(reload() + 1); } } }, "Delete" }
                }}
            }
        }
    } }
}

#[component]
pub fn ComposePostPage() -> Element {
    let auth = use_auth();
    let mut title = use_signal(String::new);
    let mut content = use_signal(String::new);
    let mut feedback = use_signal(String::new);
    let navigator = use_navigator();
    rsx! { section { class: "page auth-page", div { class: "auth-card",
        p { class: "eyebrow", "New community note" } h1 { "Write a post." }
        if let Some(token) = auth() {
            form { class: "auth-form", onsubmit: move |event| { let token = token.clone(); async move {
                event.prevent_default();
                match api::create_post(&token, crate::types::PostInput { title: title(), content: content() }).await {
                    Ok(post) => { navigator.push(Route::PostPage { id: post.id }); }
                    Err(_) => feedback.set("Your post could not be published. Check the API endpoint.".into()),
                }
            }},
                label { "Title" input { class: "field", value: "{title}", oninput: move |e| title.set(e.value()) } }
                label { "Post" textarea { class: "field comment-input", value: "{content}", oninput: move |e| content.set(e.value()) } }
                button { class: "button button-primary", r#type: "submit", "Publish post" }
                if !feedback().is_empty() { p { class: "feedback", "{feedback}" } }
            }
        } else { p { class: "signin-hint", "Sign in before writing a post." } }
    } } }
}

#[component]
pub fn ProfilePage() -> Element {
    let auth = use_auth();
    let profile = use_resource(move || {
        let token = auth();
        async move {
            match token {
                Some(token) => api::me(&token).await.ok(),
                None => None,
            }
        }
    });
    let mut current_password = use_signal(String::new);
    let mut new_password = use_signal(String::new);
    let mut feedback = use_signal(String::new);
    rsx! { section { class: "page auth-page", div { class: "auth-card",
        p { class: "eyebrow", "Profile & security" } h1 { "Your account." }
        match &*profile.read() {
            Some(Some(user)) => rsx! { div { class: "profile-summary", p { class: "profile-avatar", "{user.email.chars().next().unwrap_or('U')}" } div { strong { "{user.email}" } p { "Member #{user.id}" } } } },
            Some(None) => rsx! { p { class: "muted", "Your profile could not be loaded." } },
            None => rsx! { p { class: "muted", "Loading profile…" } },
        }
        if let Some(token) = auth() { form { class: "auth-form", onsubmit: move |event| { let token = token.clone(); async move {
            event.prevent_default();
            match api::change_password(&token, crate::types::PasswordChangeRequest { current_password: current_password(), new_password: new_password() }).await {
                Ok(_) => { current_password.set(String::new()); new_password.set(String::new()); feedback.set("Password updated.".into()); }
                Err(_) => feedback.set("Password update failed. Implement the backend endpoint first.".into()),
            }
        }},
            h2 { "Change password" }
            label { "Current password" input { class: "field", r#type: "password", value: "{current_password}", oninput: move |e| current_password.set(e.value()) } }
            label { "New password" input { class: "field", r#type: "password", value: "{new_password}", oninput: move |e| new_password.set(e.value()) } }
            button { class: "button button-primary", r#type: "submit", "Update password" }
            if !feedback().is_empty() { p { class: "feedback", "{feedback}" } }
        }}
    } } }
}
