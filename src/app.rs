use crate::error_template::{AppError, ErrorTemplate};
use leptos::*;
use leptos_meta::*;
use leptos_router::*;

pub mod routes;

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/congress-gov-surrealdb.css"/>

        // sets the document title
        <Title text="Gov Data + Leptos + SurrealDB"/>

        // content for this welcome page
        <Router fallback=|| {
            let mut outside_errors = Errors::default();
            outside_errors.insert_with_default_key(AppError::NotFound);
            view! {
                <ErrorTemplate outside_errors/>
            }
            .into_view()
        }>
            <main>
                <Routes>
                    <Route path="" view=routes::home::HomePage/>
                    <Route path="/bills" view=routes::bills::BillsPage/>
                    <Route path="/bills/:bill_type/:bill_number" view=routes::bills::bill::Bill/>
               </Routes>
            </main>
        </Router>
    }
}
