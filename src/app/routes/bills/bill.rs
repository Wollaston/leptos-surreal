use leptos::*;
use leptos_router::use_params_map;
use serde::{Deserialize, Serialize};

#[component]
pub fn Bill() -> impl IntoView {
    let params = use_params_map();
    let bill_number = move || {
        params
            .with(|params| params.get("id").cloned())
            .unwrap_or("Error loading Bill Number.".to_string())
    };
    let bill = create_resource(
        move || params().get("id").cloned().unwrap_or_default(),
        move |id| async move { get_bill(id).await },
    );

    view! {
        <div>
            <h1>"Specific Bill"</h1>
            <h1>{bill_number}</h1>
            <Transition fallback=move || view! { <p>"Loading..."</p> }>
                {move || match bill.get() {
                    None => {
                        view! { <h1>"No Bill Found."</h1>
                        }.into_view()
                    }
                    Some(bill) => match bill {
                        Err(e) => {
                            view! {
                                <pre class="error">"Server Error: " {e.to_string()}</pre>
                            }
                                .into_view()
                        }
                        Ok(bill) => {
                            view! { <h1>{bill.title}</h1> }
                        }.into_view()
                    }
                }}
            </Transition>
        </div>
    }
}

#[server]
async fn get_bill(bill_number: String) -> Result<Bill, ServerFnError> {
    let client = reqwest::Client::new();
    let token = dotenvy_macro::dotenv!("CONGRESS_GOV_API_TOKEN");

    let res = client
        .get(format!(
            "https://api.congress.gov/v3/bill/117/hr/{}?format=json&api_key={}",
            bill_number, token
        ))
        .send()
        .await?
        .text()
        .await?;

    dbg!(&res);

    let res: Response = serde_json::from_str(res.as_str())?;
    let bill = res.bill;

    dbg!("{:?}", &bill);

    Ok(bill)
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Bill {
    title: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Response {
    bill: Bill,
}
