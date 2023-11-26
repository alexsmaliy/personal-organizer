mod add_modal;
mod bookmarks;
mod command_modal;
mod home;
mod hotkeys;
mod icon;
mod left_menu;
mod login_page;
mod settings_modal;
mod top_menu;

pub(crate) mod providers;

pub(crate) use add_modal::AddModal;
pub(crate) use bookmarks::Bookmarks;
pub(crate) use command_modal::CommandModal;
pub(crate) use home::Home;
pub(crate) use hotkeys::Hotkeys;
pub(crate) use icon::Icon;
pub(crate) use left_menu::LeftMenu;
pub(crate) use login_page::LoginPage;
pub(crate) use settings_modal::SettingsModal;
pub(crate) use top_menu::TopMenu;

pub(crate) fn a_portal_is_mounted() -> bool {
    leptos::document().get_elements_by_class_name("portal").length() > 0
}