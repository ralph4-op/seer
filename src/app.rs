use yew::prelude::*;
use yew_router::prelude::*;
use yew_ethereum_provider::EthereumProvider;
use crate::pages::*;
use crate::components::sign_in::{SignIn, ConnectedWalletContext};
use crate::pages::contracts::Contracts;

#[function_component(App)]
pub fn app() -> Html {
    let connected_address = use_state(|| None::<String>);

    let connected_wallet_ctx = ConnectedWalletContext {
        address: connected_address,
    };

    html! {
        <BrowserRouter>
            <EthereumProvider>
                <ContextProvider<ConnectedWalletContext> context={connected_wallet_ctx}>
                    <Switch<Route> render={switch} />
                </ContextProvider<ConnectedWalletContext>>
            </EthereumProvider>
        </BrowserRouter>
    }
}