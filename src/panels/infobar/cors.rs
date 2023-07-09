use leptos::*;

#[component]
pub fn CorsRequirement(cx: Scope) -> impl IntoView {
    let resource = create(cx);

    let condition = resource.read(cx).unwrap_or(true);

    view! { cx,
        <Show
            when=condition
            fallback=|_| ()
        >
            <div class="error">
                <span class="iconify" data-icon="carbon:error"></span>
                <span>"CORS is enabled"</span>
            </div>
        </Show>
    }
}

pub fn create(cx: Scope) -> Resource<(), bool> {
    create_resource(cx, || (), |_| async move { true })
}
