use crate::routes::Route;
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
            if let Ok(value) = document::eval("return localStorage.getItem('rustboard.jwt') || '';")
                .join::<String>()
                .await
            {
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
            let encoded = serde_json::to_string(&value).unwrap_or_default();
            document::eval(&format!(
                "localStorage.setItem('rustboard.jwt', {encoded});"
            ));
        } else {
            document::eval("localStorage.removeItem('rustboard.jwt');");
        }
    });
    rsx! { document::Link { rel: "icon", href: FAVICON } document::Stylesheet { href: MAIN_CSS } Router::<Route> {} }
}
