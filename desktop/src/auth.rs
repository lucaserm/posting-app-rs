use dioxus::prelude::*;
pub type AuthToken = Signal<Option<String>>;
pub fn use_auth() -> AuthToken {
    use_context::<AuthToken>()
}
