use leptos::*;

#[component]
pub fn Footer() -> impl IntoView {
    view! {
        <div class="w-full bg-slate-100">
            <footer class="bg-white shadow dark:bg-gray-800">
                <div class="w-full mx-auto max-w-screen-xl p-4 md:flex md:items-center md:justify-between">
                <span class="text-sm text-gray-500 sm:text-center dark:text-gray-400">"GovData + Leptos + SurrealDB"</span>
                <ul class="flex flex-wrap items-center mt-3 text-sm font-medium text-gray-500 dark:text-gray-400 sm:mt-0">
                    <li>
                        <a href="/" class="hover:underline me-4 md:me-6">"Home"</a>
                    </li>
                </ul>
                </div>
            </footer>
        </div>
    }
}
