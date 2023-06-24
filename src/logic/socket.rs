use js_sys::{JsString, Reflect};
use leptos::*;
use wasm_bindgen_futures::JsFuture;

#[derive(Clone)]
pub struct Socket {
    app_scope: Scope,
    gateway_url: Resource<(), Option<(bool, String)>>,
}

impl Socket {
    pub fn new(cx: Scope) -> Self {
        let gateway_url = create_resource(cx, || (), |_| fetch_gateway_url());

        create_effect(cx, move |_| {
            gateway_url.with(cx, |res| {
                if let Some((from_local, url)) = res {
                    log::debug!(
                        "Gateway URL: {}\nFetched from {} source",
                        url,
                        if *from_local { "local" } else { "remote" }
                    );
                } else {
                    log::error!("Failed to fetch gateway URL");
                }
            })
        });

        Self {
            app_scope: cx,
            gateway_url,
        }
    }
}

async fn fetch_gateway_url() -> Option<(bool, String)> {
    if let Some(url) = read_gateway_local() {
        Some((true, url))
    } else {
        if let Some(window) = web_sys::window() {
            if let Ok(request) = wasm_bindgen_futures::JsFuture::from(
                window.fetch_with_str("https://discord.com/api/v10/gateway"),
            )
            .await
            {
                let request = web_sys::Request::from(request);
                if let Ok(json_promise) = request.json() {
                    let json = wasm_bindgen_futures::JsFuture::from(json_promise).await;
                    if let Some(url) = json
                        .ok()
                        .map(|json| {
                            js_sys::Reflect::get(&json, &js_sys::JsString::from("url"))
                                .map(|obj| js_sys::JsString::from(obj).as_string())
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
