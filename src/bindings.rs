use leptos::*;

pub fn setup(cx: Scope) {
    crate::panels::account::login::setup(cx);
    crate::panels::infobar::missing_intents::setup(cx);
}
