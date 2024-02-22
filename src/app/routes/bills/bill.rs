use leptos::*;
use leptos_router::use_params;
use leptos_router::Params;
use serde::{Deserialize, Serialize};

#[component]
pub fn Bill() -> impl IntoView {
    let params = use_params::<BillParams>();

    let bill = create_resource(
        move || params.get().unwrap(),
        |bill_params| async move { get_bill(bill_params.bill_type, bill_params.bill_number).await },
    );

    view! {
        <div class="bg-slate-100">
            <Transition fallback=move || view! { <p>"Loading..."</p> }>
                {move || match bill.get() {
                    None => {
                        view! { <h1>"No Bill to Load"</h1>
                        }.into_view()
                    }
                    Some(bill) => match bill {
                        Err(e) => view!{ <h1>"Error loading bill: "{e.to_string()}</h1> }.into_view(),
                        Ok(bill) => view! {
                            <h1>"Specific Bill"</h1>
                            <h1>{bill.title}</h1>
                            }.into_view()
                    }}}
            </Transition>
        </div>
    }
}

#[derive(Params, PartialEq, Eq, Clone)]
pub struct BillParams {
    bill_type: String,
    bill_number: String,
}

#[server]
async fn get_bill(bill_type: String, bill_number: String) -> Result<Bill, ServerFnError> {
    let client = reqwest::Client::new();
    let token = dotenvy_macro::dotenv!("CONGRESS_GOV_API_TOKEN");

    let res = client
        .get(format!(
            "https://api.congress.gov/v3/bill/118/{}/{}?format=json&api_key={}",
            bill_type.to_lowercase(),
            bill_number,
            token
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
