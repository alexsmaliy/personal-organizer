use leptos::*;
use leptos_router::*;
use crate::server_fns::get_bookmarks;
use crate::types::Bookmark;

pub(crate) type BookmarksResource = Resource<(), Result<Vec<Bookmark>, ServerFnError>>;

#[component]
pub(crate) fn BookmarksProvider(children: Children) -> impl IntoView {
    let bookmarks: BookmarksResource = create_resource(|| (), |_| get_bookmarks());
    provide_context(bookmarks);
    children()
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
pub(crate) struct MenuState {
    pub(crate) command_modal: CommandModalState,
    pub(crate) left_menu: LeftMenuState,
}

#[component]
pub(crate) fn MenuStateProvider(children: Children) -> impl IntoView {
    let (open, set_open) = create_signal(true);
    let left_menu = LeftMenuState { open, set_open };

    let (open, set_open) = create_signal(true);
    let (search, set_search) = create_signal(String::new());
    let (cursor, set_cursor) = create_signal(3_usize);
    let command_modal = CommandModalState { open, set_open, search, set_search, cursor, set_cursor };

    let menu_state = MenuState { command_modal, left_menu };

    provide_context(menu_state);
    children()
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
    children()
}
