use leptos::*;

#[component]
pub fn CTDCheck(cx: Scope) -> impl IntoView {
    let is_local = web_sys::window()
        .and_then(|window| window.location().hostname().ok())
        .map_or(false, |hostname| {
            hostname == "localhost" || hostname == "127.0.0.1"
        });
    let ctd_version = web_sys::window()
        .and_then(|window| {
            js_sys::Reflect::get(
                &window,
                &js_sys::JsString::from("CHAOS_THROUGH_DISCORD_VERSION"),
            )
            .ok()
        })
        .and_then(|version| version.as_f64());

    view! { cx,
        <Show when=move || !is_local && ctd_version != Some(1.0) fallback=|_| ()>
            <div class="error">
                <span class="iconify" data-icon="carbon:error"></span>
                <span>"You are missing a required userscript (or it is outdated)."</span>
                <br/>
                <br/>
                <div>
                    "Add "
                    <a href="https://raw.githubusercontent.com/helvieq499/chaos/master/chaos_through_discord.user.js">
                        "Chaos Through Discord"
                    </a> " to your "
                    <a href="https://violentmonkey.github.io/get-it/">"userscript manager"</a>
                    " for this app to operate correctly and reload the tab."
                </div>
            </div>
        </Show>
    }
}
