use leptos::*;
use leptos_router::A;
use crate::components::Icon;
use crate::components::providers::{BookmarksResource, UrlState};
use crate::types::{Bookmark, BookmarkWithTags};

#[component]
pub(crate) fn Bookmarks() -> impl IntoView {
    let bookmarks_resource: BookmarksResource = expect_context();

    let bookmarks_view_with_fallbacks = move || {
        bookmarks_resource.with(|res| match res {
            Some(Ok(bookmarks)) => {
                let (bookmarks, _) = create_signal(bookmarks.clone());
                view! {
                    <For
                        each=bookmarks
                        key=|BookmarkWithTags { bookmark, .. }| bookmark.id
                        children=move |bookmark| view! { <BookmarkWithTags bookmark /> }
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
fn BookmarkWithTags(bookmark: BookmarkWithTags) -> impl IntoView {
    let BookmarkWithTags { bookmark: Bookmark { url, title, about, archive, trash, .. }, tags } = bookmark;
    let UrlState { view, .. } = expect_context();
    view! {
        <article class="bookmark">
            <div class="left">
                <h1 class="title">
                    {title}
                    <Icon icon="star" classes="star" />
                </h1>
                <A href={format!("https://{url}")} class="url">{url}</A>
                <p class="about">{about}</p>
                <nav>{
                        tags.into_iter().map(|tag|
                            view! { <A href={format!("/{}/{}", view(), tag)} class="tag">{tag}</A> })
                        .collect_view()
                }</nav>
            </div>
            <menu class="right">
                <Icon icon="edit" classes="edit" />
                <Icon icon={if archive {"unarchive"} else {"archive"}} classes="archive" />
                {  
                    if trash {
                        view! { <Icon icon="restore_from_trash" classes="delete" /> }
                    } else {
                        ().into_view()
                    }
                }
                <Icon icon={if trash {"delete_forever"} else {"delete"}} classes="delete" />
            </menu>
        </article>
    }
}