use leptos::*;

use crate::components::Icon;
use crate::components::providers::BookmarksResource;
use crate::types::Bookmark;

#[component]
pub(crate) fn Bookmarks() -> impl IntoView {
    let bookmarks_resource: BookmarksResource =
        use_context().expect("no bookmarks resource provided by context!");

    let bookmarks_view_with_fallbacks = move || {
        bookmarks_resource.with(|res| match res {
            Some(Ok(bookmarks)) => {
                let (bookmarks, _) = create_signal(bookmarks.clone());
                view! {
                    <For
                        each=bookmarks
                        key=|bookmark| bookmark.id
                        children=move |bookmark| view! { <Bookmark bookmark /> }
                    />
                }
            },
            Some(Err(server_err)) =>
                view! {
                    <div>"error from server: "{server_err.ser().unwrap_or_default()}</div>
                }.into_view(),
            None =>
                view! {
                    <div>"none from resource"</div>
                }.into_view(),
        }
    )};

    view! {
        <section id="bookmarks">
            <Transition fallback=|| view! {"not loaded"}>
                {bookmarks_view_with_fallbacks}
            </Transition>
        </section>
    }
}

#[component]
fn Bookmark(bookmark: Bookmark) -> impl IntoView {
    let Bookmark { id: _, url, title, about, star: _, archive, trash } = bookmark;
    view! {
        <article class="bookmark">
            <div class="left">
                <h1 class="title">
                    {title}
                    <Icon icon="star" classes="star" />
                </h1>
                <a href={format!("https://{url}")} class="url">{url}</a>
                <p class="about">{about}</p>
            </div>
            <menu class="right">
                <Icon icon="edit" classes="edit" />
                <Icon icon={if archive {"unarchive"} else {"archive"}} classes="archive" />
                {  
                    if trash {
                        view! { <Icon icon="restore_from_trash" classes="delete" /> }
                    } else {
                        view! {}.into_view()
                    }
                }
                <Icon icon={if trash {"delete_forever"} else {"delete"}} classes="delete" />
            </menu>
        </article>
    }
}