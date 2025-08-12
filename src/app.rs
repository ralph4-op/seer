use yew::prelude::*;
use yew_router::prelude::*;
use yew_ethereum_provider::EthereumProvider;

use crate::components::sign_in::{SignIn, ConnectedWalletContext};
use crate::pages::contracts::Contracts;

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/sign_in")]
    SignIn,
    #[at("/contracts")]
    Contracts,
    #[at("/settings/*")]
    Settings,
    #[not_found]
    #[at("/404")]
    NotFound,
}

fn switch(routes: Route) -> Html {
    match routes {
        Route::SignIn => html! { <SignIn /> },
        Route::Contracts => html! { <Contracts /> },
        Route::NotFound => html! { <h1>{"404 - Page Not Found"}</h1> },
        Route::Home => todo!(), //will contain standard feed
        Route::Settings => todo!(),
    }
}

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