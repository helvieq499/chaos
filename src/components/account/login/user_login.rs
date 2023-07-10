use crate::logic::{client::Credentials, Client};
use leptos::*;
use std::rc::Rc;

#[derive(Default, serde::Serialize)]
struct SignIn {
    pub gift_code_sku_id: serde_json::Value,
    pub login: String,
    pub login_source: serde_json::Value,
    pub password: String,
    pub undelete: bool,
}

#[derive(serde::Deserialize)]
struct SignInSuccess {
    pub token: String,
}

#[derive(serde::Deserialize)]
struct SignInFailure {
    pub code: Option<u16>,

    #[serde(flatten)]
    pub captcha: Option<SignInFailureCaptcha>,
}

#[derive(serde::Deserialize)]
struct SignInFailureCaptcha {
    // pub captcha_key: Vec<String>,
    pub captcha_sitekey: String,
    pub captcha_service: String,
}

#[component]
pub fn UserLogin(cx: Scope) -> impl IntoView {
    let client = Client::get(cx);

    let (username, set_username) = create_signal(cx, String::new());
    let (password, set_password) = create_signal(cx, String::new());

    let (captcha, set_captcha) = create_signal::<Option<String>>(cx, None);
    let (captcha_site_key, set_captcha_site_key) = create_signal::<Option<String>>(cx, None);

    let login_action = create_action(cx, move |()| {
        sign_in(
            client.clone(),
            username,
            password,
            captcha,
            set_captcha_site_key,
        )
    });

    view! { cx,
        <div id="user_login_grid">
            <Credentials set_username set_password login_action/>
            <Captcha captcha_site_key set_captcha login_action/>
        </div>
    }
    // TODO: MFA
    // TODO: success/fail tile
}

async fn sign_in(
    client: Rc<Client>,
    username: ReadSignal<String>,
    password: ReadSignal<String>,
    captcha: ReadSignal<Option<String>>,
    set_captcha_site_key: WriteSignal<Option<String>>,
) {
    let payload = serde_json::to_string(&SignIn {
        login: username.get_untracked(),
        password: password.get_untracked(),
        ..Default::default()
    })
    .expect("valid serialization");

    let mut req = client
        .http
        .post("https://discord.com/api/v10/auth/login")
        .body(payload)
        .header("Content-Type", "application/json");

    if let Some(captcha) = captcha.get_untracked() {
        req = req.header("X-Captcha-Key", captcha);
    }

    match req.send().await {
        Ok(resp) => {
            let status = resp.status().as_u16();
            let text = resp.text().await;
            if text.is_err() {
                log::error!("Failed to read response text");
                return;
            }
            let text = text.expect("must be valid");

            match status {
                200 => serde_json::from_str::<SignInSuccess>(&text).map_or_else(
                    |_| {
                        log::error!("Failed to parse successful sign in response");
                    },
                    |success| {
                        client.credentials.update(|creds| {
                            *creds = Some(Credentials {
                                token: success.token,
                                is_bot: false,
                                message_intent: false,
                            });
                        });
                    },
                ),
                400 => {
                    if let Ok(failure) = serde_json::from_str::<SignInFailure>(&text) {
                        if let Some(code) = failure.code {
                            if code == 50035 && text.contains("INVALID_LOGIN") {
                                todo!();
                            } else {
                                log::error!("Unknown failure code {code}");
                            }
                        } else if let Some(captcha) = failure.captcha {
                            match captcha.captcha_service.as_str() {
                                "hcaptcha" => set_captcha_site_key(Some(captcha.captcha_sitekey)),
                                unknown => {
                                    log::error!("Unknown captcha provider \"{unknown}\" required");
                                }
                            }
                        } else {
                            log::error!("Unknown failure reason");
                        }
                    } else {
                        log::error!("Failed to parse failed sign in response");
                    }
                }
                _ => log::error!("Unknown status code whilst signing in"),
            }
        }
        Err(reason) => log::error!("Error occurred whilst signing in: {}", reason),
    }
}

#[component]
fn Credentials(
    cx: Scope,
    set_username: WriteSignal<String>,
    set_password: WriteSignal<String>,
    login_action: Action<(), ()>,
) -> impl IntoView {
    let set_from_event = |target: WriteSignal<String>| move |ev| target(event_target_value(&ev));
    view! { cx,
        <div>
            <label for="login">"Email"</label>
            <input name="login" on:change=set_from_event(set_username)/>
            <br/>
            <label for="password">"Password"</label>
            <input name="password" type="password" on:change=set_from_event(set_password)/>
            <br/>
            <button on:click=move |_| login_action.dispatch(())>"Login"</button>
        </div>
    }
}

#[component]
fn Captcha(
    cx: Scope,
    captcha_site_key: ReadSignal<Option<String>>,
    set_captcha: WriteSignal<Option<String>>,
    login_action: Action<(), ()>,
) -> impl IntoView {
    let captcha = create_node_ref(cx);
    captcha.on_load(cx, move |element: HtmlElement<html::Div>| {
        if let Some(sitekey) = captcha_site_key.get() {
            element
                .set_attribute("data-sitekey", &sitekey)
                .expect("set attribute");
        }
    });

    let on_click = move |_| {
        if let Some(element) = captcha.get() {
            if let Some(child) = element.children().get_with_index(1) {
                if let Some(value) = child.node_value() {
                    set_captcha(Some(value));
                }
            }
        }

        login_action.dispatch(());
    };

    // HACK: inline script to render hcaptcha
    view! { cx,
        <div>
            <Show when=move || captcha_site_key.get().is_some() fallback=|_| ()>
                <div node_ref=captcha id="user_login_captcha"></div>
                <script>"hcaptcha.render(user_login_captcha)"</script>
                <button on:click=on_click>"Submit Captcha"</button>
            </Show>
        </div>
    }
}
