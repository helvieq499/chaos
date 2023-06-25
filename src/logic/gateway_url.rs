use js_sys::{JsString, Reflect};
use leptos::*;
use wasm_bindgen_futures::JsFuture;

pub fn resource(cx: Scope) -> Resource<(), Option<(bool, String)>> {
    create_resource(cx, || (), |_| fetch_gateway_url())
}

async fn fetch_gateway_url() -> Option<(bool, String)> {
    if let Some(url) = read_gateway_local() {
        Some((true, url))
    } else {
        if let Some(window) = web_sys::window() {
            if let Ok(request) =
                JsFuture::from(window.fetch_with_str("https://discord.com/api/v10/gateway")).await
            {
                let request = web_sys::Request::from(request);
                if let Ok(json_promise) = request.json() {
                    let json = JsFuture::from(json_promise).await;
                    if let Some(url) = json
                        .ok()
                        .map(|json| {
                            Reflect::get(&json, &JsString::from("url"))
                                .map(|obj| JsString::from(obj).as_string())
                                .ok()
                                .flatten()
                        })
                        .flatten()
                    {
                        crate::utils::local_storage::get()
                            .map(|local_storage| local_storage.set("gateway_url", &url));

                        Some((false, url.to_string()))
                    } else {
                        log::error!("Failed to get URL");
                        None
                    }
                } else {
                    log::error!("Failed to parse JSON from response");
                    None
                }
            } else {
                log::error!("Network error while fetching gateway address");
                None
            }
        } else {
            log::info!("window was none");
            None
        }
    }
}

fn read_gateway_local() -> Option<String> {
    crate::utils::local_storage::get()
        .map(|local_storage| local_storage.get("gateway_url").ok().flatten())
        .flatten()
}
