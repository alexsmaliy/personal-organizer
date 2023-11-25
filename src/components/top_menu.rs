use std::rc::Rc;
use leptos::*;
use leptos_router::{NavigateOptions, use_navigate};
use crate::components::Icon;
use crate::components::providers::MenuState;

#[component]
pub(crate) fn TopMenu() -> impl IntoView {
    let MenuState { add_modal, left_menu, settings_modal, .. } = expect_context();

    let go = Rc::new(use_navigate());
    // TODO: go("/all") should use settings
    view! {
        <nav id="top-menu">
            <section class="left">
                <Icon icon="menu" on:click=move |_| left_menu.toggle() />
                {
                    let go = Rc::clone(&go);
                    view! { <Icon icon="home" on:click=move |_| go("/all", NavigateOptions::default()) /> }
                }
            </section>
            <section class="right">
                <Icon icon="logout" />
                {
                    let go = Rc::clone(&go);
                    view! { <Icon icon="help" on:click=move |_| go("/help", NavigateOptions::default()) /> }
                }
                <Icon icon="add" on:click=move |_| add_modal.open() />
                <Icon icon="settings" classes="settings" on:click=move |_| settings_modal.open() />
                <Icon icon="account_circle" classes="account" />
            </section>
        </nav>
    }
}