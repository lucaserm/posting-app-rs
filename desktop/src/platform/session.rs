use dioxus::prelude::document;

const TOKEN_KEY: &str = "rustboard.jwt";

/// Web and desktop use WebView local storage. Mobile can replace this module
/// with Keychain/Keystore-backed storage without affecting shared UI code.
pub async fn load() -> Option<String> {
    document::eval(&format!(
        "return localStorage.getItem('{TOKEN_KEY}') || '';"
    ))
    .join::<String>()
    .await
    .ok()
    .filter(|token| !token.is_empty())
}

pub fn save(token: &str) {
    let encoded = serde_json::to_string(token).unwrap_or_default();
    document::eval(&format!("localStorage.setItem('{TOKEN_KEY}', {encoded});"));
}

pub fn clear() {
    document::eval(&format!("localStorage.removeItem('{TOKEN_KEY}');"));
}
