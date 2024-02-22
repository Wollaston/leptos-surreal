use leptos::*;

#[component]
pub fn FeedsPage() -> impl IntoView {
    view! {
        <div class="bg-slate-100">
            <h1>"Feeds Page"</h1>
            <a href="feeds/bills">"Bills"</a>
        </div>
    }
}
