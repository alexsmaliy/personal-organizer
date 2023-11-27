use std::ops::Deref;
use leptos::*;
use leptos::html::{Form, Input, Textarea};
use wasm_bindgen::JsCast;
use web_sys::{Element, Event, HtmlElement, KeyboardEvent};
use crate::components::providers::{AddModalState, MenuState};
use crate::types::{Bookmark, BookmarkWithTags};

#[component]
pub(crate) fn AddModal() -> impl IntoView {
    let MenuState { add_modal: AddModalState { open, .. }, .. } = expect_context();
    
    let force_local_rendering = create_local_resource(|| (), |_| async { true });

    view! {
        <Suspense fallback=|| ().into_view() >
            {if force_local_rendering.get().is_some() { ().into_view() } else { ().into_view() }}
            <Show when=open>
                <Portal>
                    <AddModalInner />
                </Portal>
            </Show>
        </Suspense>
    }
}

#[component]
fn AddModalInner() -> impl IntoView {
    let form_ref = create_node_ref::<Form>();
    let input_ref0 = create_node_ref::<Input>();
    let input_ref1 = create_node_ref::<Input>();
    let input_ref2 = create_node_ref::<Textarea>();
    let input_ref3 = create_node_ref::<Input>();

    let MenuState { add_modal: AddModalState {
        set_open,
        title, set_title,
        url, set_url,
        about, set_about,
        tags, set_tags,
        .. }, .. } = expect_context();

    form_ref.on_load(move |inner| {
        inner.deref().set_attribute("class", "portal");
        request_animation_frame(move || {
            input_ref0.get_untracked().unwrap().focus();
        });
    });

    let click_listener = window_event_listener(ev::mousedown, move |event| {
        let modal_ref = form_ref.get_untracked().expect("add modal menu should exist by now");
        let modal: &HtmlElement = modal_ref.deref();
        let clicked_element = event.target().unwrap().unchecked_into::<Element>();
        if !modal.contains(Some(&clicked_element)) {
            set_open(false);
        }
    });

    on_cleanup(move || click_listener.remove());

    let Δtitle = move |e: Event| set_title(String::from(event_target_value(&e).trim()));
    let Δurl = move |e: Event| set_url(String::from(event_target_value(&e).trim()));
    let Δabout = move |e: Event| set_about(String::from(event_target_value(&e).trim()));
    let Δtags = move |e: Event| set_tags(String::from(event_target_value(&e).trim()));

    let (focused, set_focused) = create_signal(0_usize);

    let do_focus = move |index: usize| match index {
        0 => input_ref0.get_untracked().unwrap().focus(), 1 => input_ref1.get_untracked().unwrap().focus(),
        2 => input_ref2.get_untracked().unwrap().focus(), 3 => input_ref3.get_untracked().unwrap().focus(),
        _ => unreachable!(),
    };

    let Δkey = move |e: KeyboardEvent| {
        e.stop_immediate_propagation();
        let f = focused();
        match e.key().as_ref() {
            "Tab" => {
                e.prevent_default();
                let next = if e.shift_key() { (f + 3) % 4 } else { (f + 1) % 4 };
                set_focused(next);
                do_focus(next);
            },
            "Escape" => set_open(false),
            _ => {},
        };
    };

    let submit = move |_| {
        let bookmark = Bookmark {
            id: uuid::Uuid::default(),
            title: title().trim().into(),
            url: url().trim().into(),
            about: about().trim().into(),
            star: false,
            archive: false,
            trash: false,
        };
        use itertools::Itertools;
        let tags: Vec<_> = tags().split(',').map(str::trim).map(String::from).unique().collect();
        let bookmark = BookmarkWithTags { bookmark, tags };
    };

    view! {
      <form id="modal" node_ref=form_ref on:keydown=Δkey>
          <section>
              <fieldset>
                  <label html_for="name-field">"Name"</label>
                  <input id="name-field" type_="text" value=title on:input=Δtitle placeholder="title" node_ref=input_ref0 />
              </fieldset>
              <fieldset>
                  <label html_for="url-field">"URL"</label>
                  <input id="url-field" type_="text" value=url on:input=Δurl placeholder="url" node_ref=input_ref1 />
              </fieldset>
              <fieldset>
                  <label html_for="about-field">"About"</label>
                  <textarea id="about-field" value=about on:input=Δabout placeholder="about" node_ref=input_ref2 />
              </fieldset>
              <fieldset>
                  <label html_for="tags-field">"Tags"</label>
                  <input id="tags-field" type_="text" value=tags on:input=Δtags placeholder="tags" node_ref=input_ref3 />
              </fieldset>
          </section>
          <button class="buttongray" type_="submit" on:submit=submit>
              "Submit"
          </button>
      </form>
    }
}