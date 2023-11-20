use leptos::*;
use leptos_router::*;

use crate::components::Icon;
use crate::components::providers::{LeftMenuState, MenuState};

#[component]
pub(crate) fn LeftMenu() -> impl IntoView {
    let MenuState { left_menu: LeftMenuState { open, .. }, .. } =
        use_context().expect("did not find menu state in context");

    view! {
        <menu id="left-menu" class={move || if open() { "on" } else { "off" }}>
            <A href="/all" class={"item all"}>
                <Icon icon="apps" />
                <label>all</label>
            </A>
            <A href="/inbox" class={"item inbox"}>
                <Icon icon="inbox" />
                <label>inbox</label>
            </A>
            <A href="/star" class={"item star"}>
                <Icon icon="star" />
                <label>star</label>
            </A>
            <A href="/archive" class={"item archive"}>
                <Icon icon="inventory_2" />
                <label>archive</label>
            </A>
            <A href="/trash" class={"item trash"}>
                <Icon icon="delete" />
                <label>trash</label>
            </A>
            <HorizontalLine />
            <A href="/search" class={"item search"}>
                <Icon icon="search" />
                <label>search</label>
            </A>
            <A href="/stats" class={"item stats"}>
                <Icon icon="bar_chart" />
                <label>stats</label>
            </A>
        </menu>
    }
}

#[component]
fn HorizontalLine() -> impl IntoView {
    view! {
        <div class="horizontal-line" />
    }
}