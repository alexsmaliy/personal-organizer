use std::ops::Deref;
use std::rc::Rc;
use leptos::*;
use leptos::html::{Input, Nav};
use leptos_router::{NavigateOptions, use_navigate};
use wasm_bindgen::JsCast;
use web_sys::{Element, Event, HtmlElement, KeyboardEvent};
use crate::components::Icon;
use crate::components::providers::{CommandModalState, MenuState};
use crate::types::AppView;

#[component]
pub(crate) fn CommandModal() -> impl IntoView {
    let MenuState { command_modal: CommandModalState { open, .. }, .. } = expect_context();

    let force_local_rendering = create_local_resource(|| (), |_| async { true });

    view! {
        <Suspense fallback=|| ().into_view() >
            {if force_local_rendering.get().is_some() { ().into_view() } else { ().into_view() }}
            <Show when=open>
                <Portal>
                    <CommandModalInner />
                </Portal>
            </Show>
        </Suspense>
    }
}

#[component]
fn CommandModalInner() -> impl IntoView {
    let MenuState { add_modal, command_modal, left_menu, .. } = expect_context();

    let nav_ref: NodeRef<Nav> = create_node_ref();
    let input_ref: NodeRef<Input> = create_node_ref();

    let click_listener = window_event_listener(ev::click, move |event| {
        let nav_ref = nav_ref.get_untracked().expect("nav should exist by now");
        let nav: &HtmlElement = nav_ref.deref();
        let clicked_element = event.target().unwrap().unchecked_into::<Element>();
        if !nav.contains(Some(&clicked_element)) {
            command_modal.close();
        }
    });

    on_cleanup(move || click_listener.remove());

    input_ref.on_load(move |inner| {
        inner.deref().set_attribute("class", "portal");
        request_animation_frame(move || {   
            inner.focus();
            command_modal.set_cursor.set(0);
        });
    });

    let go = Rc::new(use_navigate());

    let menu_items = store_value({
        use AppView::*;
        let views = vec![
            (ALL,     view! { <Icon icon="apps"        classes="all" /> }),
            (INBOX,   view! { <Icon icon="inbox"       classes="inbox" /> }),
            (STAR,    view! { <Icon icon="star"        classes="star" /> }),
            (TRASH,   view! { <Icon icon="delete"      classes="trash" /> }),
            (ARCHIVE, view! { <Icon icon="inventory_2" classes="archive" /> }),
            (SEARCH,  view! { <Icon icon="search"      classes="search" /> }),
        ];
        
        let items: Vec<MenuCategory> = vec![
            MenuCategory {
                index: 0,
                section: "Actions".into(),
                items: vec![
                    MenuItem {
                        hotkey: Some("q".into()),
                        icon: view! { <Icon icon="add" /> },
                        index: None,
                        label: "Add bookmark".into(),
                        action: create_action(move |_| async move {
                            command_modal.close();
                            add_modal.open();
                        }),
                    },
                    MenuItem {
                        hotkey: Some("m".into()),
                        icon: view! { <Icon icon="menu_open" /> },
                        index: None,
                        label: "Toggle left menu".into(),
                        action: create_action(move |_| async move {
                            left_menu.toggle();
                            command_modal.close();
                        }),
                    },
                ]
            },
            MenuCategory {
                index: 1,
                section: "Navigate".into(),
                items: views.into_iter().map(|(view_name, icon_el)| {
                    let go = Rc::clone(&go);
                    MenuItem {
                        hotkey: None,
                        index: None,
                        label: view_name.to_string(),
                        icon: icon_el,
                        action: create_action(move |_| {
                            let go = Rc::clone(&go);
                            async move {
                                command_modal.close();
                                let path = &format!("/{view_name}");
                                let opts = NavigateOptions::default();
                                go(path, opts);
                            }
                        }),
                    }
                }).collect(),
            },
            MenuCategory {
                index: 2,
                section: "Settings".into(),
                items: vec![
                    MenuItem {
                        hotkey: None,
                        icon: view! { <Icon icon="settings" /> },
                        index: None,
                        label: "Open settings".into(),
                        action: create_action(move |_| async move {
                            command_modal.close();
                        }),
                    },
                    MenuItem {
                        hotkey: None,
                        icon: view! { <Icon icon="light" /> },
                        index: None,
                        label: "Toggle dark mode".into(),
                        action: create_action(move |_| async move {
                            command_modal.close();
                        }),
                    },
                ],
            }
        ];

        items
    });

    let flat_menu = Signal::derive(move || {
        let search = command_modal.search.get().to_lowercase();
        let items: Vec<_> = menu_items().into_iter()
            .flat_map(|MenuCategory { section, items, index }| {
                items.into_iter().map(move |item| (item, section.clone(), index))
            })
            .filter(|(item, _, _)| {
                item.label.to_lowercase().contains(&search)
            })
            .enumerate()
            .map(|(index, (item, section, cat_index))|
                (section, cat_index, MenuItem { index: Some(index), ..item })
            )
            .collect();
        items
    });

    let tall_menu = Signal::derive(move || {
        let mut items: Vec<_> = flat_menu().into_iter()
            .fold(
                hashbrown::HashMap::new(),
                |mut acc, (section, cat_index, item)| {
                    acc.entry((section, cat_index)).or_insert(vec![]).push(item);
                    acc
                }
            )
            .into_iter()
            .map(|((section, cat_index), items)|
                MenuCategory { section, items, index: cat_index }
            )
            .collect();
        items.sort_by_cached_key(|m| m.index);
        items
    });

    let CommandModalState { search, set_search, cursor, set_cursor, .. } = command_modal;

    let Δsearch = move |e: Event| {
        set_search(event_target_value(&e));
        set_cursor(0);
    };

    let Δkey = move |e: KeyboardEvent| {
        e.stop_immediate_propagation();
        let max = flat_menu().len();
        let c = cursor();
        match (e.key().as_ref(), e.shift_key()) {
            ("Escape", _) => {
                e.prevent_default();
                command_modal.close();
            },
            ("Tab", false) | ("ArrowDown", _) => {
                e.prevent_default();
                set_cursor((c + 1) % max);
            },
            ("Tab", true) | ("ArrowUp", _) => {
                e.prevent_default();
                set_cursor((c + max - 1) % max);
            },
            ("Enter", _) => {
                e.prevent_default();
                flat_menu()[c].2.action.dispatch(());
            },
            _ => {},
        }
    };

    view! {
        <nav id="command" node_ref=nav_ref on:keydown=Δkey>
            <input placeholder="search" value=search on:input=Δsearch node_ref=input_ref />
            <Show when=move || !tall_menu().is_empty() fallback=|| view! { <menu><h1>"No results"</h1></menu> }>
                <For
                    each=store_value(tall_menu())
                    key=|category| category.index
                    children=move |MenuCategory { section, items, .. }| view! {
                        <menu>
                            <h1>{section}</h1>
                            <ul>
                                <For
                                    each=store_value(items)
                                    key=|menu_item| menu_item.index.unwrap()
                                    children=move |MenuItem { index: i, action, icon, label, hotkey }| view! {
                                        <li
                                            class:active=create_memo(move |_| cursor() == i.unwrap())
                                            on:click=move |_| action.dispatch(())
                                            on:mousemove=move |_| set_cursor(i.unwrap())>
                                            {icon}
                                            <span>{label}</span>
                                            {
                                                hotkey.map_or_else(
                                                     || ().into_view(),
                                                    |h| view! { <kbd>{h}</kbd> }.into_view())
                                            }
                                        </li>
                                    }
                                />
                            </ul>
                        </menu>
                    }
                />
            </Show>
        </nav>
    }
}

#[derive(Clone)]
pub(crate) struct MenuItem {
    action: Action<(), ()>,
    hotkey: Option<String>,
    index: Option<usize>,
    icon: View,
    label: String,
}

impl PartialEq for MenuItem {
    fn eq(&self, other: &Self) -> bool {
        self.hotkey == other.hotkey &&
        self.index == other.index &&
        self.icon == other.icon &&
        self.label == other.label
    }
}

impl std::fmt::Debug for MenuItem {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("MenuItem")
         .field("label", &self.label)
         .field("icon", &self.icon)
         .field("index", &self.index)
         .field("hotkey", &self.hotkey)
         .finish()
    }
}

#[derive(Clone, PartialEq)]
pub(crate) struct MenuCategory {
    section: String,
    index: usize,
    items: Vec<MenuItem>,
}

impl std::fmt::Debug for MenuCategory {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("MenuCategory")
         .field("index", &self.index)
         .field("section", &self.section)
         .field("items", {
             &self.items.iter()
                  .map(|m| format!("{m:?}"))
                  .collect::<Vec<_>>()
                  .join("\n")})
         .finish()
    }
}