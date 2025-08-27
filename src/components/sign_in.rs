use yew::prelude::*;
use yew_ethereum_provider::{EthereumProvider, EthereumContext};
use serde::{Deserialize, Serialize};
use web_sys::console;
use wasm_bindgen_futures::spawn_local;
use yew_router::prelude::*;
use crate::Route;

// This context will hold the connected Ethereum address
#[derive(Clone, PartialEq)]
pub struct ConnectedWalletContext {
    pub address: UseStateHandle<Option<String>>,
}

#[function_component(SignIn)]
pub fn sign_in() -> Html {
    let ethereum = use_context::<EthereumContext>().expect("Ethereum context not found");
    let connected_wallet_ctx = use_context::<ConnectedWalletContext>().expect("Connected Wallet context not found");

    let signature = use_state(|| None::<String>);
    let error = use_state(|| None::<String>);
    let navigator = use_navigator().unwrap();

    let on_connect = {
        let ethereum = ethereum.clone();
        let connected_wallet_ctx = connected_wallet_ctx.clone();
        let error = error.clone();
        let navigator = navigator.clone();

        Callback::from(move |_| {
            let ethereum = ethereum.clone();
            let connected_wallet_ctx = connected_wallet_ctx.clone();
            let error = error.clone();
            let navigator = navigator.clone();

            spawn_local(async move {
                match ethereum.connect().await {
                    Ok(accounts) => {
                        if let Some(address) = accounts.get(0) {
                            console::log_1(&format!("Connected to MetaMask with address: {}", address).into());
                            connected_wallet_ctx.address.set(Some(address.to_string()));

                            // Check if user exists in OrbitDB
                            let user_exists = wasm_bindgen::JsValue::from(js_sys::global().get("getUserProfile").call1(&js_sys::global(), &wasm_bindgen::JsValue::from_str(&address.to_string())).unwrap()).as_bool().unwrap_or(false);

                            if user_exists {
                                navigator.push(&Route::Contracts); // Navigate to contracts page if user exists
                            } else {
                                navigator.push(&Route::CreateAccount); // Navigate to create account page if user does not exist
                            }
                        } else {
                            error.set(Some("No accounts found after connection.".to_string()));
                        }
                    }
                    Err(e) => {
                        console::error_1(&format!("Failed to connect to MetaMask: {:?}", e).into());
                        error.set(Some(format!("Failed to connect to MetaMask: {:?}", e)));
                    }
                }
            });
        })
    };

    let on_sign_message = {
        let ethereum = ethereum.clone();
        let signature = signature.clone();
        let error = error.clone();

        Callback::from(move |_| {
            let ethereum = ethereum.clone();
            let signature = signature.clone();
            let error = error.clone();

            spawn_local(async move {
                let message = "Please sign this message to verify your identity for Zcash wallet.";
                let signer = ethereum.signer();

                match signer.sign_message(message).await {
                    Ok(sig) => {
                        signature.set(Some(sig));
                        console::log_1(&"Message signed successfully!".into());
                    }
                    Err(e) => {
                        console::error_1(&format!("Failed to sign message: {:?}", e).into());
                        error.set(Some(format!("Failed to sign message: {:?}", e)));
                    }
                }
            });
        })
    };

    html! {
        <div class="sign-in-container">
            <h1>{"Connect Your Wallet"}</h1>
            {if connected_wallet_ctx.address.is_none() {
                html! {
                    <button onclick={on_connect} class="connect-button">
                        {"Connect MetaMask"}
                    </button>
                }
            } else {
                html! {
                    <div class="wallet-info">
                        <p>{"Connected Address: "}{connected_wallet_ctx.address.as_ref().unwrap()}</p>
                        {if signature.is_none() {
                            html! {
                                <button onclick={on_sign_message} class="sign-message-button">
                                    {"Sign Message for Zcash"}
                                </button>
                            }
                        } else {
                            html! {
                                <p>{"Message Signed! You can now proceed to Contracts page."}</p>
                            }
                        }}
                    </div>
                }
            }}
            {if let Some(err) = &*error {
                html! { <p style="color: red;">{err}</p> }
            } else {
                html! {}
            }}
        </div>
    }
}