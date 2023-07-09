use leptos::*;

#[component]
pub fn CorsRequirement(cx: Scope) -> impl IntoView {
    let resource = create(cx);
    let visibility = move || match resource.read(cx) {
        Some(false) => "",
        _ => "none",
    };

    view! { cx,
        <div class="error" style=("display", visibility)>
            <span class="iconify" data-icon="carbon:error"></span>
            <span>"CORS is enabled"</span>
        </div>
    }
}

pub fn create(cx: Scope) -> Resource<(), bool> {
    create_resource(cx, || (), |_| async move { true })
}
