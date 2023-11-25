use std::ops::Deref;
use leptos::*;
use leptos::html::{Div, Form};
use wasm_bindgen::JsCast;
use web_sys::{Element, HtmlElement};
use crate::components::providers::MenuState;

#[component]
pub(crate) fn SettingsModal() -> impl IntoView {
    let MenuState { settings_modal, .. } = expect_context();
    
    let force_local_rendering = create_local_resource(|| (), |_| async { true });

    view! {
        <Suspense fallback=|| view! {}.into_view()>
            {if force_local_rendering.get().is_some() { view! {}.into_view() } else { view! {}.into_view() }}
            <Show when=move || settings_modal.is_open() || settings_modal.is_closing()>
                <Portal>
                    <SettingsModalInner />
                </Portal>
            </Show>
        </Suspense>
    }
}

#[component]
fn SettingsModalInner() -> impl IntoView {
    let MenuState { settings_modal, .. } = expect_context();

    let form_ref: NodeRef<Form> = create_node_ref();
    let overlay_ref: NodeRef<Div> = create_node_ref();

    let click_listener = window_event_listener(ev::mousedown, move |event| {
        let form_ref = form_ref.get_untracked().expect("settings modal menu should exist by now");
        let form: &HtmlElement = form_ref.deref();
        let clicked_element = event.target().unwrap().unchecked_into::<Element>();
        if !form.contains(Some(&clicked_element)) {
            let overlay_ref = overlay_ref.get_untracked().expect("settings modal overlay should exist");
            overlay_ref.deref().style().set_property("background-color", "transparent");
            settings_modal.close();
        }
    });

    on_cleanup(move || click_listener.remove());

    form_ref.on_load(move |inner| {
        inner.deref().set_attribute("class", "portal");
    });

    view! {
        <div id="settings-modal-overlay" node_ref=overlay_ref>
            <form id="settings" node_ref=form_ref class=move || if settings_modal.is_closing() { "exit" } else { "enter" }>
                <h2>"Settings"</h2>
                <button class="buttongray">"Save"</button>
            </form>
        </div>
    }
}