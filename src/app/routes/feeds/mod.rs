use leptos::*;
use leptos_router::Outlet;

pub mod bills;
pub mod main;

#[component]
pub fn Feeds() -> impl IntoView {
    view! {
        <Outlet/>
    }
}
