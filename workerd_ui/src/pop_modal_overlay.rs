use gloo::events::EventListener;
use web_sys::{
    Element,
    HtmlElement,
    wasm_bindgen::{
        JsCast,
        prelude::Closure,
    },
};

use super::*;

#[derive(Properties, PartialEq)]
pub struct PopModalOverlayProps {
    #[prop_or_default]
    pub children: Html,

    #[prop_or_default]
    pub onclick: Callback<bool>,

    #[prop_or_else(|| true)]
    pub disable_exit_onclick: bool,
}


#[function_component]
pub fn PopModalOverlay(props: &PopModalOverlayProps) -> Html {
    let overlay_ref = use_node_ref();

    {
        let disable_exit_onclick = props.disable_exit_onclick;
        let onclick = props.onclick.clone();
        let overlay_ref = overlay_ref.clone();
        use_effect_with(overlay_ref, move |overlay_ref| {
            let mut overlay_listener = None;

            if let Some(overlay) = overlay_ref.cast::<Element>() {
                if !disable_exit_onclick {
                    let listener = EventListener::new(
                        &overlay,
                        "click",
                        move |_e: &Event| onclick.emit(false));
                    web_sys::console::log_1(&"Checked!".into());

                    overlay_listener = Some(listener);
                }
            }

            move || drop(overlay_listener)
        });
    }

    html! {
        <div ref={overlay_ref} class="fixed inset-0 z-10 w-screen overflow-y-auto">
            {props.children.clone()}
        </div>
    }
}
