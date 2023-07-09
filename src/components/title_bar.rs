use leptos::*;
use leptos_router::*;

#[component]
pub fn TitleBar(cx: Scope) -> impl IntoView {
    let version = move || {
        #[cfg(not(debug_assertions))]
        {
            let url = crate::build_info::GIT_COMMIT_HASH.map_or_else(
                || "".to_string(),
                |hash| format!("{}/commit/{hash}", crate::build_info::PKG_REPOSITORY),
            );

            view! { cx,
                <a
                    href=url
                    style="margin-left: auto"
                >
                    <span>"Version "</span>
                    {crate::build_info::GIT_COMMIT_HASH_SHORT}
                </a>
            }
        }
    };

    view! { cx,
        <div id="title_bar">
            <A href="/account">"Account"</A>
            <A href="/guilds">"Guilds"</A>
            {version}
        </div>
    }
}
