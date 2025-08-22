use yew::prelude::*;
use yew_ethereum_provider::EthereumContext;
use wasm_bindgen_futures::spawn_local;
use web_sys::console;
use crate::components::sign_in::ConnectedWalletContext;

// Import the ZcashWallet from the wallet_gen module
// This path might need adjustment based on how wallet_gen is exposed in the main crate
use crate::wallet_gen::zcash::ZcashWallet;

#[function_component(Contracts)]
pub fn Contracts() -> Html {
    let connected_wallet_ctx = use_context::<ConnectedWalletContext>().expect("Connected Wallet context not found");
    let zcash_address = use_state(|| None::<String>);
    let error_message = use_state(|| None::<String>);

    // Effect hook to generate Zcash address when component mounts or wallet connects
    use_effect_with_deps(move |connected_wallet_ctx| {
        let zcash_address = zcash_address.clone();
        let error_message = error_message.clone();
        let connected_address = connected_wallet_ctx.address.clone();

        spawn_local(async move {
            if let Some(eth_address) = (*connected_address).clone() {
                console::log_1(&format!("Ethereum address connected: {}", eth_address).into());
                // In a real application, you might derive the Zcash key from the Ethereum key
                // For now, we'll just generate a new Zcash wallet
                let wallet = ZcashWallet::new();
                let address = wallet.get_unified_address();
                zcash_address.set(Some(address));
            } else {
                error_message.set(Some("Please connect your Ethereum wallet first.".to_string()));
            }
        });
        || ()
    }, connected_wallet_ctx.clone());

    html! {
        <div class="contracts-container">
            <h1>{"Your  Contracts"}</h1>

            {if let Some(eth_addr) = (*connected_wallet_ctx.address).clone() {
                html! { <p>{"Connected Ethereum Address: "}{eth_addr}</p> }
            } else {
                html! { <p style="color: orange;">{"Ethereum wallet not connected."}</p> }
            }}

            <h2>{"Zcash Wallet"}</h2>
            {if let Some(addr) = (*zcash_address).clone() {
                html! { <p>{"Your Zcash Unified Address: "}{addr}</p> }
            } else if let Some(err) = (*error_message).clone() {
                html! { <p style="color: red;">{err}</p> }
            } else {
                html! { <p>{"Generating Zcash address..."}</p> }
            }}

            // Add more components or logic here for other contracts/assets
            <div>
                <h2>{"Your Orders"}</h2>
                // List of contracts or contract components can be added here
            </div>
        </div>
    }
}