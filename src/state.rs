use axum::extract::FromRef;
use leptos::LeptosOptions;
use leptos_router::RouteListing;
use surrealdb::{engine::remote::ws::Client, Surreal};

#[derive(FromRef, Debug, Clone)]
pub struct AppState {
    pub leptos_options: LeptosOptions,
    pub db: Surreal<Client>,
    pub routes: Vec<RouteListing>,
}
