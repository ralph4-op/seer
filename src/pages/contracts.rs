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
// app.rs
let call_contract = {
    Callback::from(move |_| {
        spawn_local(async move {
            let result = invoke("call_contract", JsValue::NULL).await;
            console::log!("Contract call result:", result);
        });
    })
};

html! {
    <button onclick={call_contract}>{"Call Contract"}</button>
}