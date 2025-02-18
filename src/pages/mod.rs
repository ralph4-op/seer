use yew::prelude::*;
use yew_router::prelude::*;
use gloo::utils::window;
use wasm_bindgen::UnwrapThrowExt;

#[derive(Routable, PartialEq, Clone, Debug)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/user/:username")]
    UserProfile { username: String },
    #[at("/group/:group_name")]
    Group { group_name: String },
    #[at("/trend/:trend_type/:trend_name")]
    Trend { trend_type: String, trend_name: String },
    #[not_found]
    #[at("/404")]
    NotFound,
}

#[function_component(Home)]
fn home() -> Html {
    html! { <h1>{"Welcome to the App"}</h1> }
}

#[function_component(UserProfile)]
fn user_profile({ username }: &UserProfileProps) -> Html {
    html! { <h1>{format!("User Profile: {}", username)}</h1> }
}

#[derive(Properties, PartialEq)]
struct UserProfileProps {
    username: String,
}

#[function_component(Group)]
fn group({ group_name }: &GroupProps) -> Html {
    html! { <h1>{format!("Group: {}", group_name)}</h1> }
}

#[derive(Properties, PartialEq)]
struct GroupProps {
    group_name: String,
}

#[function_component(Trend)]
fn trend({ trend_type, trend_name }: &TrendProps) -> Html {
    let trend_display = match trend_type.as_str() {
        "financial" => format!("Financial Trend: ${}", trend_name),
        "general" => format!("General Trend: #{}", trend_name),
        _ => format!("Unknown Trend: {}", trend_name),
    };
    html! { <h1>{trend_display}</h1> }
}

#[derive(Properties, PartialEq)]
struct TrendProps {
    trend_type: String,
    trend_name: String,
}

#[function_component(NotFound)]
fn not_found() -> Html {
    html! { <h1>{"404 - Page Not Found"}</h1> }
}

fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! { <Home /> },
        Route::UserProfile { username } => html! { <UserProfile username={username} /> },
        Route::Group { group_name } => html! { <Group group_name={group_name} /> },
        Route::Trend { trend_type, trend_name } => html! { <Trend trend_type={trend_type} trend_name={trend_name} /> },
        Route::NotFound => html! { <NotFound /> },
    }
}