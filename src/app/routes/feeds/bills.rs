use leptos::*;
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::fmt::Display;

use crate::domain::bills::BillType;

#[component]
pub fn BillsFeed() -> impl IntoView {
    let items = create_resource(|| (), |_| async move { get_bills_feed().await });

    view! {
        <div class="bg-slate-100">
            <h1>"Bills Feed"</h1>
            <Transition
                fallback=move || view! { <p>"Loading..."</p> }
            >
                <ul>
                {move || match items.get() {
                    None => view! { <h1>"No items found in Feed."</h1> }.into_view(),
                    Some(Err(_)) => view! { <p>"Error loading feed."</p> }.into_view(),
                    Some(Ok(items)) => {
                        {items.into_iter()
                            .map(|bill| view! { <BillsFeedCard bill/> })
                            .collect_view()}                }
                            }}
                </ul>
            </Transition>
        </div>
    }
}

#[component]
fn BillsFeedCard(bill: BillItem) -> impl IntoView {
    view! {
            <ErrorBoundary
                // the fallback receives a signal containing current errors
                fallback=|_| view! {
                    <li class="bg-slate-500 text-blue-700 p-4 m-4">
                        <p>"Error loading item."</p>
                    </li>
                }
            >
            <li class="m-4 p-4 bg-white border border-gray-200 rounded-lg shadow dark:bg-gray-800 dark:border-gray-700">
                <h5 class="mb-2 text-2xl font-bold tracking-tight text-gray-900 dark:text-white">{bill.bill_type.clone()}" "{bill.bill_number.clone()}" ("{bill.bill_version}")"</h5>
                <p class="mb-3 font-normal text-gray-700 dark:text-gray-400">{bill.title}</p>
                <a href={format!("/bills/{}/{}", bill.bill_type.unwrap(), bill.bill_number.unwrap())} class="inline-flex items-center px-3 py-2 text-sm font-medium text-center text-white bg-blue-700 rounded-lg hover:bg-blue-800 focus:ring-4 focus:outline-none focus:ring-blue-300 dark:bg-blue-600 dark:hover:bg-blue-700 dark:focus:ring-blue-800">
                "Get Details"
                <svg class="rtl:rotate-180 w-3.5 h-3.5 ms-2" aria-hidden="true" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 14 10">
                    <path stroke="currentColor" stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M1 5h12m0 0L9 1m4 4L9 9"/>
                </svg>
                </a>
            </li>
         </ErrorBoundary>
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum FeedErrors {
    ParseError,
    MatchError,
}

impl Display for FeedErrors {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FeedErrors::ParseError => write!(f, "Error parsing Bill Number."),
            FeedErrors::MatchError => write!(f, "Error matching Bill Type."),
        }
    }
}

impl std::error::Error for FeedErrors {}

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
            let escaped_title = htmlize::unescape(title);
            let bill_item = parse_bill_item(escaped_title.as_ref());
            bill_item
        })
        .collect();

    Ok(bill_items)
}

fn parse_bill_item(input: &str) -> BillItem {
    println!("{:?}", input);
    let re = Regex::new(
        r"(?<bill_type>[a-zA-Z. ]+)( |\u{a0})(?<bill_number>\d+)( |\u{a0})\((?<bill_version>\w+)\) - (?<title>.*$)",
    )
    .unwrap();
    let caps = re.captures(input).unwrap();
    let bill_type = match caps["bill_type"]
        .replace(['.', ' '], "")
        .to_uppercase()
        .as_str()
    {
        "HR" => Ok(BillType::HR),
        "S" => Ok(BillType::S),
        "HRES" => Ok(BillType::HRES),
        "SRES" => Ok(BillType::SRES),
        "HJRES" => Ok(BillType::HJRES),
        "SJRES" => Ok(BillType::SJRES),
        "HCONRES" => Ok(BillType::HCONRES),
        "SCONRES" => Ok(BillType::SCONRES),
        _ => Err(FeedErrors::MatchError),
    };
    let bill_number = caps["bill_number"]
        .parse::<i32>()
        .map_err(|_| FeedErrors::ParseError);
    let bill_version = caps["bill_version"].to_string();
    let title = caps["title"].to_string();
    BillItem {
        bill_type,
        bill_number,
        bill_version,
        title,
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BillItem {
    bill_type: Result<BillType, FeedErrors>,
    bill_number: Result<i32, FeedErrors>,
    bill_version: String,
    title: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BillItems {
    bills: Vec<BillItem>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bill_type_parsing() {
        let input = "H.R. 7261 (IH) - Reimagining Inclusive Arts Education Act";
    }
}
