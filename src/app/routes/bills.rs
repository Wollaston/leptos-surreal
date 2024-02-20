use leptos::*;
use leptos_router::ActionForm;
use serde::{Deserialize, Serialize};

use crate::error_template::ErrorTemplate;

pub mod bill;

#[component]
pub fn BillsPage() -> impl IntoView {
    let get_bills_action = create_server_action::<GetBills>();

    let bills = get_bills_action.value();

    view! {
        <div>
            <h1>"Bills Page"</h1>
            <ActionForm action=get_bills_action>
                <input type="submit" value="Get Bills"/>
            </ActionForm>
            <div>
            <Transition fallback=move || view! { <p>"Loading..."</p> }>
                <ErrorBoundary fallback=|errors| {
                    view! { <ErrorTemplate errors=errors/> }
                }>
                    {move || {
                        let existing_bills = {
                            move || {
                               bills
                                    .get()
                                    .map(move |bills| match bills {
                                        Err(e) => {
                                            view! {
                                                <pre class="error">"Server Error: " {e.to_string()}</pre>
                                            }
                                                .into_view()
                                        }
                                        Ok(bills) => {
                                            if bills.bills.is_empty() {
                                                view! { <p>"No bills were found."</p> }.into_view()
                                            } else {
                                               bills.bills
                                                    .into_iter()
                                                    .map(move |bill| {
                                                        view! {
                                                            <li>
                                                                {bill.number}" - "{bill.title}
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
                    view! { <ul>{existing_bills}</ul> }
                    }}
                   </ErrorBoundary>
            </Transition>
    </div>
        </div>
    }
}

#[server]
async fn get_bills() -> Result<Bills, ServerFnError> {
    let client = reqwest::Client::new();

    let token = dotenvy_macro::dotenv!("CONGRESS_GOV_API_TOKEN");

    let bills = client
        .get(format!(
            "https://api.congress.gov/v3/bill?format=json&offset=0&limit=10&api_key={}",
            token
        ))
        .send()
        .await?
        .text()
        .await?;

    let bills: Bills = serde_json::from_str(bills.as_str())?;

    dbg!("{:?}", &bills);

    Ok(bills)
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Bills {
    bills: Vec<Bill>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Bill {
    number: String,
    title: String,
}
