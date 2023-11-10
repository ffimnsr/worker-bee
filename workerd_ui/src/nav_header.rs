use super::*;

#[function_component]
pub fn NavHeader() -> Html {
    html! {
        <header class="bg-white">
            <nav class="mx-auto flex items-center justify-between p-6 lg:px-8" aria-label="global">
                <div class="flex lg:flex-1">
                    <a href="#" class="-m-1.5 p-1.5">
                        {"WorkerBee Console"}
                    </a>
                </div>
                <div class="hidden lg:flex lg:flex-1 lg:justify-end">
                    <a href="#" class="rounded-md bg-indigo-600 px-3 py-2 text-sm font-semibold text-white shadow-sm hover:bg-indigo-500 focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-indigo-600">{"Save"}</a>
                </div>
            </nav>
        </header>
    }
}
