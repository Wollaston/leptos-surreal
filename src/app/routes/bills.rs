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
                                                                <BillCard bill/>
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

#[component]
fn BillCard(bill: Bill) -> impl IntoView {
    view! {
       <div class="max-w-sm p-6 bg-white border border-gray-200 rounded-lg shadow dark:bg-gray-800 dark:border-gray-700">
            <h5 class="mb-2 text-2xl font-bold tracking-tight text-gray-900 dark:text-white">{&bill.bill_type}" "{&bill.number}</h5>
            <p class="mb-3 font-normal text-gray-700 dark:text-gray-400">{bill.title}</p>
            <a href={format!("bills/{}/{}", bill.bill_type, bill.number)} class="inline-flex items-center px-3 py-2 text-sm font-medium text-center text-white bg-blue-700 rounded-lg hover:bg-blue-800 focus:ring-4 focus:outline-none focus:ring-blue-300 dark:bg-blue-600 dark:hover:bg-blue-700 dark:focus:ring-blue-800">
            "Get Details"
               <svg class="rtl:rotate-180 w-3.5 h-3.5 ms-2" aria-hidden="true" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 14 10">
                   <path stroke="currentColor" stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M1 5h12m0 0L9 1m4 4L9 9"/>
               </svg>
           </a>
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
    #[serde(rename = "type")]
    bill_type: String,
}
