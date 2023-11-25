use std::time::Duration;
use leptos::*;
use leptos_dom::helpers::set_timeout;
use leptos_router::use_params_map;
use crate::server_fns::get_bookmarks;
use crate::types::Bookmark;

pub(crate) type BookmarksResource = Resource<(), Result<Vec<Bookmark>, ServerFnError>>;

#[component]
pub(crate) fn BookmarksProvider(children: Children) -> impl IntoView {
    let bookmarks: BookmarksResource = create_resource(|| (), |_| get_bookmarks());
    provide_context(bookmarks);
    return children();
}

#[derive(Clone, Copy)]
pub(crate) struct LeftMenuState {
    pub(crate) open: ReadSignal<bool>,
    set_open: WriteSignal<bool>,
}

impl LeftMenuState {
    pub(crate) fn open(&self) { self.set_open.set(true); }
    pub(crate) fn close(&self) { self.set_open.set(false); }
    pub(crate) fn toggle(&self) { self.set_open.update(|m| *m = !*m); }
}

#[derive(Clone, Copy)]
pub(crate) struct CommandModalState {
    pub(crate) open: ReadSignal<bool>,
    set_open: WriteSignal<bool>,
    pub(crate) search: ReadSignal<String>,
    pub(crate) set_search: WriteSignal<String>,
    pub(crate) cursor: ReadSignal<usize>,
    pub(crate) set_cursor: WriteSignal<usize>,
}

impl CommandModalState {
    pub(crate) fn open(&self) { self.set_open.set(true); }
    pub(crate) fn close(&self) { self.set_open.set(false); }
    pub(crate) fn toggle(&self) { self.set_open.update(|x| *x = !*x); }
}

#[derive(Clone, Copy)]
pub(crate) struct AddModalState {
    pub(crate) open: ReadSignal<bool>,
    pub(crate) set_open: WriteSignal<bool>,
    pub(crate) title: ReadSignal<String>,
    pub(crate) set_title: WriteSignal<String>,
    pub(crate) url: ReadSignal<String>,
    pub(crate) set_url: WriteSignal<String>,
    pub(crate) about: ReadSignal<String>,
    pub(crate) set_about: WriteSignal<String>,
    pub(crate) tags: ReadSignal<String>,
    pub(crate) set_tags: WriteSignal<String>,
}

impl AddModalState {
    pub(crate) fn open(&self) { self.set_open.set(true); }
    pub(crate) fn close(&self) { self.set_open.set(false); }
}

#[derive(Clone, Copy, PartialEq)]
pub(crate) enum ModalCondition {
    OPEN, CLOSING, CLOSED,
}

#[derive(Clone, Copy)]
pub(crate) struct SettingsModalState {
    pub(crate) open: ReadSignal<ModalCondition>,
    pub(crate) set_open: WriteSignal<ModalCondition>,
}

impl SettingsModalState {
    pub(crate) fn open(&self) { self.set_open.set(ModalCondition::OPEN); }
    pub(crate) fn is_closing(&self) -> bool { self.open.get() == ModalCondition::CLOSING }
    pub(crate) fn is_open(&self) -> bool { self.open.get() == ModalCondition::OPEN }
    pub(crate) fn close(&self) {
        self.set_open.set(ModalCondition::CLOSING);
        let self2 = self.clone();
        set_timeout(move || self2.set_open.set(ModalCondition::CLOSED), Duration::from_millis(200));
    }
}

#[derive(Clone, Copy)]
pub(crate) struct MenuState {
    pub(crate) add_modal: AddModalState,
    pub(crate) command_modal: CommandModalState,
    pub(crate) left_menu: LeftMenuState,
    pub(crate) settings_modal: SettingsModalState,
}

#[component]
pub(crate) fn MenuStateProvider(children: Children) -> impl IntoView {
    let (open, set_open) = create_signal(false);
    let (title, set_title) = create_signal(String::new());
    let (url, set_url) = create_signal(String::new());
    let (about, set_about) = create_signal(String::new());
    let (tags, set_tags) = create_signal(String::new());
    let add_modal = AddModalState { open, set_open, title, set_title, url, set_url, about, set_about, tags, set_tags };

    let (open, set_open) = create_signal(false);
    let (search, set_search) = create_signal(String::new());
    let (cursor, set_cursor) = create_signal(0);
    let command_modal = CommandModalState { open, set_open, search, set_search, cursor, set_cursor };

    let (open, set_open) = create_signal(true);
    let left_menu = LeftMenuState { open, set_open };

    let (open, set_open) = create_signal(ModalCondition::CLOSED);
    let settings_modal = SettingsModalState { open, set_open };

    let menu_state = MenuState { add_modal, command_modal, left_menu, settings_modal };

    provide_context(menu_state);
    return children();
}

#[derive(Clone, Copy)]
pub(crate) struct UrlState {
    pub view: Signal<String>,
    pub tags: Signal<String>,
}

#[component]
pub(crate) fn UrlStateProvider(children: Children) -> impl IntoView {
    let params = use_params_map();
    let view = Signal::derive(move || params.with(|p| p.get("view").cloned().unwrap()));
    let tags = Signal::derive(move || params.with(|p| p.get("tags").cloned().unwrap_or_default()));
    let url_state = UrlState { view, tags };
    provide_context(url_state);
    return children();
}
