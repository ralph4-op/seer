use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

// #[wasm_bindgen]
// extern "C" {
//     #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"])]
//     async fn invoke(cmd: &str, args: JsValue) -> JsValue;
// }

// #[derive(Serialize, Deserialize)]
// struct GreetArgs<'a> {
//     name: &'a str,
// }

#[function_component(App)]
pub fn app() -> Html {
    {
    html! {
        <main class="container">
       
<h1>{"Hi!"}</h1>
   
            // <form class="row" onsubmit={greet}>
            //     <input id="greet-input" ref={greet_input_ref} placeholder="Enter a name..." />
            //     <button type="submit">{"Greet"}</button>
            // </form>

            // <p><b>{ &*greet_msg }</b></p>
        </main>
    }
}
}
