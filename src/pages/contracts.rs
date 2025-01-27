use yew::prelude::*;

#[function_component(YourContractsPage)]
pub fn YourContractsPage() -> Html {
    html! {
        <div>
            <h1>{"Your Portfolio"}</h1>
            // Add more components or logic here
        </div>
        <div>
        <h2>{"Your Orders"}</h2>
            // List of contracts or contract components can be added here
        </div>
    }
}