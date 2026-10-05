use crate::{platform::session, routes::Route};
use dioxus::prelude::*;
const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.css");
#[component]
pub fn App() -> Element {
    let mut token = use_signal(|| None::<String>);
    let mut ready = use_signal(|| false);
    use_context_provider(|| token);
    use_effect(move || {
        spawn(async move {
            if let Some(value) = session::load().await {
                if !value.is_empty() {
                    token.set(Some(value));
                }
            }
            ready.set(true);
        });
    });
    use_effect(move || {
        let value = token();
        if !ready() {
            return;
        }
        if let Some(value) = value {
            session::save(&value);
        } else {
            session::clear();
        }
    });
    rsx! { document::Link { rel: "icon", href: FAVICON } document::Stylesheet { href: MAIN_CSS } Router::<Route> {} }
}
