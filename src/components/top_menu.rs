use leptos::*;
use crate::components::Icon;
use crate::components::providers::MenuState;

#[component]
pub(crate) fn TopMenu() -> impl IntoView {
    let MenuState { settings_modal, .. } = use_context().expect("menu state not found in context!");

    view! {
        <nav id="top-menu">
            <section class="left">
                <Icon icon="menu" />
                <Icon icon="home" />
            </section>
            <section class="right">
                <Icon icon="logout" />
                <Icon icon="help" />
                <Icon icon="add" />
                <Icon icon="settings" classes="settings" on:click=move |_| settings_modal.open()/>
                <Icon icon="account_circle" classes="account" />
            </section>
        </nav>
    }
}