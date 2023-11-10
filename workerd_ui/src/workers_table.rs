use web_sys::{
    wasm_bindgen::{prelude::Closure, JsCast},
    Element, HtmlElement,
};

use super::*;

enum WorkerKind {
    ServiceWorkerScript,
    Modules,
    Inherit,
}

#[function_component]
pub fn WorkersTable() -> Html {
    let show_modal = use_state(|| false);
    let onclick = {
        let show_modal = show_modal.clone();
        move |_: MouseEvent| {
            show_modal.set(true);
        }
    };

    let close_modal = {
        let show_modal = show_modal.clone();
        move |_: MouseEvent| {
            show_modal.set(false);
        }
    };

    let overlay_onclick = Callback::from({
        let show_modal = show_modal.clone();
        move |show: bool| {
            show_modal.set(show);
        }
    });

    // {
    //     let backdrop_ref = backdrop_ref.clone();
    //     use_effect_with(*show_modal, |show_modal, backdrop_ref| {
    //         if *show_modal {
    //             let backdrop = backdrop_ref
    //                 .cast::<HtmlElement>()
    //                 .expect("div_ref not attached to div element");
    //         }
    //         // let backdrop = backdrop_ref
    //         //     .cast::<HtmlElement>()
    //         //     .expect("div_ref not attached to div element");

    //         // let listener = Closure::<dyn Fn(Event)>::wrap(Box::new(|_| {
    //         //     web_sys::console::log_1(&"Clicked!".into());
    //         // }));

    //         // backdrop.add_event_listener_with_callback("click", listener.as_ref().unchecked_ref())
    //         //     .unwrap();

    //         // move || {
    //         //     backdrop.remove_event_listener_with_callback(
    //         //         "click",
    //         //         listener.as_ref().unchecked_ref(),
    //         //     )
    //         //     .unwrap();
    //         // }
    //     });
    // }

    let items = (1..=10).collect::<Vec<_>>();

    html! {
        <>
            <PopModal show={*show_modal}>
                <div class="relative z-10" aria-labelledby="modal-title" role="dialog" aria-modal="true">
                    <div class="fixed inset-0 bg-gray-500 bg-opacity-75 transition-opacity"></div>
                    <PopModalOverlay onclick={overlay_onclick}>
                        <div class="flex min-h-full items-end justify-center p-4 text-center sm:items-center sm:p-0">
                            <div
                                class="relative transform overflow-hidden rounded-lg bg-white px-4 pb-4 pt-5 text-left shadow-xl transition-all sm:my-8 sm:w-full sm:max-w-sm sm:p-6">
                                <div class="space-y-4">
                                    <div>
                                        <label for="worker-name" class="block text-sm font-medium leading-6 text-gray-900">{"Name"}</label>
                                        <div class="mt-2">
                                            <input
                                                type="text"
                                                name="worker-name"
                                                id="worker-name"
                                                class="block w-full rounded-md border-0 py-1.5 text-gray-900 shadow-sm ring-1 ring-inset ring-gray-300 placeholder:text-gray-400 focus:ring-2 focus:ring-inset focus:ring-indigo-600 sm:text-sm sm:leading-6"
                                            />
                                        </div>
                                    </div>
                                    <div>
                                        <label for="worker-type" class="block text-sm font-medium leading-6 text-gray-900">{"Location"}</label>
                                        <select
                                            id="worker-type"
                                            name="worker-type"
                                            class="mt-2 block w-full rounded-md border-0 py-1.5 pl-3 pr-10 text-gray-900 ring-1 ring-inset ring-gray-300 focus:ring-2 focus:ring-indigo-600 sm:text-sm sm:leading-6"
                                        >
                                            <option>{"United States"}</option>
                                            <option>{"Canada"}</option>
                                            <option>{"Mexico"}</option>
                                        </select>
                                    </div>
                                    <div>
                                        <label for="worker-compatibility-date" class="block text-sm font-medium leading-6 text-gray-900">{"Compatibility Date"}</label>
                                        <div class="mt-2">
                                            <input
                                                type="text"
                                                name="worker-compatibility-date"
                                                id="worker-compatibility-date"
                                                class="block w-full rounded-md border-0 py-1.5 text-gray-900 shadow-sm ring-1 ring-inset ring-gray-300 placeholder:text-gray-400 focus:ring-2 focus:ring-inset focus:ring-indigo-600 sm:text-sm sm:leading-6"
                                            />
                                        </div>
                                    </div>
                                    <div>
                                        <label for="worker-compatibility-flags" class="block text-sm font-medium leading-6 text-gray-900">{"Compatibility Flags"}</label>
                                        <div class="mt-2">
                                            <input
                                                type="text"
                                                name="worker-compatibility-flags"
                                                id="worker-compatibility-flags"
                                                class="block w-full rounded-md border-0 py-1.5 text-gray-900 shadow-sm ring-1 ring-inset ring-gray-300 placeholder:text-gray-400 focus:ring-2 focus:ring-inset focus:ring-indigo-600 sm:text-sm sm:leading-6"
                                            />
                                        </div>
                                    </div>
                                </div>
                                <div class="mt-5 sm:mt-6">
                                    <button type="button"
                                        onclick={close_modal}
                                        class="inline-flex w-full justify-center rounded-md bg-indigo-600 px-3 py-2 text-sm font-semibold text-white shadow-sm hover:bg-indigo-500 focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-indigo-600"
                                    >
                                        {"Go back to dashboard"}
                                    </button>
                                </div>
                            </div>
                        </div>
                    </PopModalOverlay>
                </div>
            </PopModal>
            <div class="pt-10 px-4 sm:px-6 lg:px-8">
                <div class="sm:flex sm:items-center">
                    <div class="sm:flex-auto">
                        <h1 class="text-base font-semibold leading-6 text-gray-900">{"Workers"}</h1>
                        <p class="mt-2 text-sm text-gray-700">
                            {"A list of all the workers installed on the host system."}
                        </p>
                    </div>
                    <div class="mt-4 sm:ml-16 sm:mt-0 sm:flex-none">
                        <button type="button"
                            {onclick}
                            class="block rounded-md bg-indigo-600 px-3 py-2 text-center text-sm font-semibold text-white shadow-sm hover:bg-indigo-500 focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-indigo-600">
                            {"Add worker"}
                        </button>
                    </div>
                </div>

                <div class="mt-8 flow-root">
                    <div class="-mx-4 -my-2 overflow-x-auto sm:-mx-6 lg:-mx-8">
                        <div class="inline-block min-w-full py-2 align-middle sm:px-6 lg:px-8">
                            <div class="overflow-hidden shadow ring-1 ring-black ring-opacity-5 sm:rounded-lg">
                                <table class="min-w-full divide-y divide-gray-300">
                                    <thead class="bg-gray-50">
                                        <tr>
                                            <th scope="col"
                                                class="py-3.5 pl-4 pr-3 text-left text-sm font-semibold text-gray-900 sm:pl-6">
                                                {"Name"}
                                            </th>
                                            <th scope="col" class="px-3 py-3.5 text-left text-sm font-semibold text-gray-900">
                                                {"Worker Type"}
                                            </th>
                                            <th scope="col" class="px-3 py-3.5 text-left text-sm font-semibold text-gray-900">
                                                {"Compatibility Date"}
                                            </th>
                                            <th scope="col" class="px-3 py-3.5 text-left text-sm font-semibold text-gray-900">
                                                {"Compatibility Flags"}
                                            </th>
                                            <th scope="col" class="relative py-3.5 pl-3 pr-4 sm:pr-6">
                                                <span class="sr-only">{"Edit"}</span>
                                            </th>
                                        </tr>
                                    </thead>
                                    <tbody class="divide-y divide-gray-200 bg-white">
                                        {
                                            items.into_iter()
                                                .map(|_| {
                                                    html! {
                                                        <tr>
                                                            <td class="whitespace-nowrap py-4 pl-4 pr-3 text-sm font-medium text-gray-900 sm:pl-6">
                                                                {"hello-world"}
                                                            </td>
                                                            <td class="whitespace-nowrap px-3 py-4 text-sm text-gray-500">
                                                                {"Service Worker Script"}
                                                            </td>
                                                            <td class="whitespace-nowrap px-3 py-4 text-sm text-gray-500">
                                                                {"2023-10-30"}
                                                            </td>
                                                            <td class="whitespace-nowrap px-3 py-4 text-sm text-gray-500">
                                                                {" "}
                                                            </td>
                                                            <td
                                                                class="relative whitespace-nowrap py-4 pl-3 pr-4 text-right text-sm font-medium sm:pr-6">
                                                                <a href="#" class="text-indigo-600 hover:text-indigo-900">{"Edit"}</a>
                                                            </td>
                                                        </tr>
                                                    }
                                                }).collect::<Html>()
                                        }
                                    </tbody>
                                </table>
                            </div>
                        </div>
                    </div>
                </div>
            </div>
        </>
    }
}
