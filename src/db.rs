use leptos::*;
use surrealdb::engine::remote::ws::Client;
use surrealdb::Surreal;

pub fn db() -> Result<Surreal<Client>, ServerFnError> {
    use_context::<Surreal<Client>>()
        .ok_or_else(|| ServerFnError::ServerError("Pool missing.".into()))
}
