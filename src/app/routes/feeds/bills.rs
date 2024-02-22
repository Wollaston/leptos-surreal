use leptos::*;
use serde::{Deserialize, Serialize};

#[component]
pub fn BillsFeed() -> impl IntoView {
    let items = create_resource(|| (), |_| async move { get_bills_feed().await });

    view! {
        <div class="bg-slate-100">
            <h1>"Bills Feed"</h1>
            <Transition
                fallback=move || view! { <p>"Loading..."</p> }
            >
                {move || match items.get() {
                    None => view! { <h1>"No items found in Feed."</h1> }.into_view(),
                    Some(Err(_)) => view! { <p>"Error loading feed."</p> }.into_view(),
                    Some(Ok(items)) => {
                        {items.into_iter()
                            .map(|item| view! { <li>{item.title}</li>})
                            .collect_view()}                }
                            }}
            </Transition>
        </div>
    }
}

#[server]
async fn get_bills_feed() -> Result<Vec<BillItem>, ServerFnError> {
    use rss::Channel;

    let content = reqwest::get("https://www.govinfo.gov/rss/bills.xml")
        .await?
        .bytes()
        .await?;
    let channel = Channel::read_from(&content[..])?;
    let items = channel.into_items();
    let bill_items: Vec<BillItem> = items
        .into_iter()
        .map(|item| {
            let title = item.title.unwrap_or("No title found for Bill.".to_string());
            let escaped_title = html_escape::decode_html_entities(title.as_str());
            BillItem {
                title: escaped_title.into(),
            }
        })
        .collect();

    Ok(bill_items)
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BillItem {
    title: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BillItems {
    bills: Vec<BillItem>,
}
