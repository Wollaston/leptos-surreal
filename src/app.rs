use crate::error_template::{AppError, ErrorTemplate};
use leptos::*;
use leptos_meta::*;
use leptos_router::*;
use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {


        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/congress-gov-surrealdb.css"/>

        // sets the document title
        <Title text="Welcome to Leptos"/>

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
                    <Route path="" view=HomePage/>
                </Routes>
            </main>
        </Router>
    }
}

/// Renders the home page of your application.
#[component]
fn HomePage() -> impl IntoView {
    // Creates a reactive value to update the button
    let (count, set_count) = create_signal(0);
    let on_click = move |_| set_count.update(|count| *count += 1);

    let add_person = create_server_multi_action::<AddPerson>();

    // list of people is loaded from the server in reaction to changes
    let people = create_resource(move || (add_person.version().get()), move |_| get_people());
    view! {
        <h1>"Welcome to Leptos!"</h1>
        <button on:click=on_click>"Click Me: " {count}</button>
        <div>
            <MultiActionForm action=add_person>
                <label>"Add a Person" <input type="text" name="title"/></label>
                <input type="submit" value="Add"/>
            </MultiActionForm>
        </div>
        <div>            <Transition fallback=move || view! { <p>"Loading..."</p> }>
                <ErrorBoundary fallback=|errors| {
                    view! { <ErrorTemplate errors=errors/> }
                }>
                    {move || {
                        let existing_people = {
                            move || {
                               people
                                    .get()
                                    .map(move |people| match people {
                                        Err(e) => {
                                            view! {
                                                <pre class="error">"Server Error: " {e.to_string()}</pre>
                                            }
                                                .into_view()
                                        }
                                        Ok(people) => {
                                            if people.is_empty() {
                                                view! { <p>"No people were found."</p> }.into_view()
                                            } else {
                                               people
                                                    .into_iter()
                                                    .map(move |person| {
                                                        view! {
                                                            <li>
                                                                {person.first}" "{person.last}
                                                            </li>
                                                        }
                                                    })
                                                    .collect_view()
                                            }
                                        }
                                    })
                                    .unwrap_or_default()
                            }
                        };
                    view! { <ul>{existing_people}</ul> }
                    }};
                   </ErrorBoundary>
            </Transition>
    </div>
    }
}

#[server]
async fn add_person() -> Result<Vec<Record>, ServerFnError> {
    use self::ssr::db;
    let db = db()?;

    let created: Vec<Record> = db
        .create("person")
        .content(Person {
            first: "Tobie".to_string(),
            last: "Morgan Hitchcock".to_string(),
        })
        .await?;
    dbg!(&created);
    Ok(created)
}

#[server]
async fn get_people() -> Result<Vec<Person>, ServerFnError> {
    use self::ssr::db;
    let db = db()?;

    let people: Vec<Person> = db.select("person").await?;

    dbg!(&people);
    Ok(people)
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Record {
    #[allow(dead_code)]
    id: Thing,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Person {
    pub first: String,
    pub last: String,
}

#[cfg(feature = "ssr")]
pub mod ssr {
    use leptos::*;
    use surrealdb::engine::remote::ws::Client;
    use surrealdb::Surreal;

    pub fn db() -> Result<Surreal<Client>, ServerFnError> {
        use_context::<Surreal<Client>>()
            .ok_or_else(|| ServerFnError::ServerError("Pool missing.".into()))
    }
}
