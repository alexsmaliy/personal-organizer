use leptos::*;
use leptos_router::{use_navigate, NavigateOptions};
use wasm_bindgen::JsCast;
use web_sys::{HtmlInputElement, EventTarget};
use crate::components::a_portal_is_mounted;
use crate::components::providers::MenuState;

#[component]
pub(crate) fn Hotkeys() -> impl IntoView {
    let MenuState { add_modal, command_modal, left_menu, settings_modal } = expect_context();
    
    let go = use_navigate();

    let click_listener = window_event_listener(ev::keydown, move |ev| {
        if !a_portal_is_mounted() {
            match ev.key().as_ref() {
                "h" => (!event_target::<EventTarget>(&ev).has_type::<HtmlInputElement>())
                            .then(|| go("/home", NavigateOptions::default())), // TODO: get home from settings
                "k" => ev.ctrl_key()
                            .then(|| command_modal.open()),
                "m" => (!event_target::<EventTarget>(&ev).has_type::<HtmlInputElement>())
                            .then(|| left_menu.toggle()),
                "q" => (!event_target::<EventTarget>(&ev).has_type::<HtmlInputElement>())
                            .then(|| add_modal.open()),
                "?" => (!event_target::<EventTarget>(&ev).has_type::<HtmlInputElement>())
                            .then(|| go("/help", NavigateOptions::default())),
                something_else => Some(logging::log!("you pressed {something_else}, but it didn't do anything"))
            };
        }
    });

    on_cleanup(move || click_listener.remove());

    return ();
}