use yew::prelude::*;
use yew_ethereum_provider::{EthereumProvider, EthereumContext};
use serde::{Deserialize, Serialize};
use web_sys::console;
use wasm_bindgen_futures::spawn_local;

#[derive(Serialize, Deserialize)]
struct UsernameRequest {
    signature: String,
    username: String,
}

#[function_component(SignIn)]
pub fn sign_in() -> Html {
    let ethereum = use_context::<EthereumContext>().expect("Ethereum context not found");

    let signature = use_state(|| None::<String>);
    let username = use_state(|| String::new());
    let error = use_state(|| None::<String>);

    let on_sign = {
        let ethereum = ethereum.clone();
        let signature = signature.clone();
        Callback::from(move |_| {
            let ethereum = ethereum.clone();
            let signature = signature.clone();

            spawn_local(async move {
                let message = "Please sign this message to verify your identity.";
                let signer = ethereum.signer();

                match signer.sign_message(message).await {
                    Ok(sig) => {
                        signature.set(Some(sig));
                    }
                    Err(e) => {
                        console::error_1(&format!("Failed to sign message: {:?}", e).into());
                    }
                }
            });
        })
    };

    let on_username_submit = {
        let signature = signature.clone();
        let username = username.clone();
        let error = error.clone();

        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();

            if let Some(sig) = &*signature {
                let username = username.to_string();

                spawn_local(async move {
                    let request = UsernameRequest {
                        signature: sig.clone(),
                        username: username.clone(),
                    };

                    // Call JavaScript function to add user profile to OrbitDB
                    let js_value = serde_wasm_bindgen::to_value(&request).unwrap();
                    let result = js_sys::Promise::from(wasm_bindgen::JsValue::from(
                        web_sys::window()
                            .unwrap()
                            .eval(&format!("addUserProfile('{}', '{}')", sig, username))
                            .unwrap(),
                    ))
                    .await;

                    match result {
                        Ok(_) => {
                            console::log_1(&"Username registered successfully!".into());
                            // Redirect or update state as needed
                        }
                        Err(e) => {
                            console::error_1(&format!("Failed to register username: {:?}", e).into());
                            error.set(Some("Failed to register username.".to_string()));
                        }
                    }
                });
            } else {
                error.set(Some("Please sign the message first.".to_string()));
            }
        })
    };

    html! {
        <div>
            <h1>{"Sign In"}</h1>
            <button onclick={on_sign}>{"Sign Message"}</button>
            {if signature.is_some() {
                html! {
                    <form onsubmit={on_username_submit}>
                        <input
                            type="text"
                            placeholder="Choose a username"
                            value={(*username).clone()}
                            oninput={Callback::from(move |e: InputEvent| {
                                let input = e.target_unchecked_into::<web_sys::HtmlInputElement>();
                                username.set(input.value());
                            })}
                        />
                        <button type="submit">{"Submit Username"}</button>
                    </form>
                }
            } else {
                html! {}
            }}
            {if let Some(err) = &*error {
                html! { <p style="color: red;">{err}</p> }
            } else {
                html! {}
            }}
        </div>
    }
}