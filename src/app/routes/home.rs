use leptos::*;
use leptos_router::MultiActionForm;
use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;

use crate::error_template::ErrorTemplate;

/// Renders the home page of your application.
#[component]
pub fn HomePage() -> impl IntoView {
    // Creates a reactive value to update the button
    let (count, set_count) = create_signal(0);
    let on_click = move |_| set_count.update(|count| *count += 1);

    let add_person = create_server_multi_action::<AddPerson>();

    // list of people is loaded from the server in reaction to changes
    let people = create_resource(move || (add_person.version().get()), move |_| get_people());
    view! {
    <div class="bg-slate-100">
        <h1 class="text-blue-700 text-2xl">"Welcome to Leptos!"</h1>
        <button on:click=on_click>"Click Me: " {count}</button>
        <div>
            <MultiActionForm action=add_person>
                <label>"First Name" <input type="text" name="first"/></label>
                <label>"Last Name" <input type="text" name="last"/></label>
                <input type="submit" value="Add"/>
            </MultiActionForm>
        </div>
        <div>
            <Transition fallback=move || view! { <p>"Loading..."</p> }>
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
                    }}
                   </ErrorBoundary>
            </Transition>
        </div>
    </div>
    }
}

#[server]
async fn add_person(first: String, last: String) -> Result<Vec<Record>, ServerFnError> {
    use crate::db::db;
    let db = db()?;

    let created: Vec<Record> = db.create("person").content(Person { first, last }).await?;
    dbg!(&created);
    Ok(created)
}

#[server]
async fn get_people() -> Result<Vec<Person>, ServerFnError> {
    use crate::db::db;
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
