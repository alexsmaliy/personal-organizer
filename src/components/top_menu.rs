use leptos::*;
use crate::components::Icon;

#[component]
pub(crate) fn TopMenu() -> impl IntoView {
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
                <Icon icon="settings" classes="settings" />
                <Icon icon="account_circle" classes="account" />
            </section>
        </nav>
    }
}