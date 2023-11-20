use leptos::*;
use leptos_router::*;

use crate::components::{Bookmarks, CommandModal, LeftMenu, TopMenu};
use crate::components::providers::{BookmarksProvider, BookmarksResource, CommandModalState, MenuState, MenuStateProvider, UrlStateProvider, UrlState};
use crate::server_fns::get_bookmarks;
use crate::types::Bookmark;

#[component]
pub(crate) fn Home() -> impl IntoView {
    // let params = use_params_map();
    // let view = Signal::derive(move || params.with(|p| p.get("view").cloned().unwrap()));
    // let tags = Signal::derive(move || params.with(|p| p.get("tags").cloned().unwrap_or_default()));
    // let bookmarks_res: BookmarksResource = create_resource(||(), |_| get_bookmarks());


    // let loading = bookmarks_res.loading();
    // let notloading = leptos::Signal::derive(move || !loading());

    // let bookmarks = move || {
    //     match bookmarks_res.get() {
    //         None => vec![],
    //         Some(result) => match result {
    //             Err(e) => {
    //                 logging::error!("[loading bookmarks]: {e}");
    //                 vec![]
    //             },
    //             Ok(b) => b,
    //         }
    //     }
    // };
    // let bstring = move || bookmarks().iter().map(|b| b.to_string()).collect::<Vec<String>>().join("\n");
    // let refetch = move |_| bookmarks_res.refetch();
    // let add_local = move |_e: leptos::ev::MouseEvent| bookmarks_res.update(|bs| match bs {
    //     Some(Ok(v)) => {
    //         let b = Bookmark { id: uuid::Uuid::default(), url: "example.com".into(), title: "example title".into(),
    //                             about: "example about".into(), star: true, archive: true, trash: true };
    //         v.push(b);
    //     },
    //     _ => {},
    // });
    use crate::types::AppView::*;
    let views = [ALL.as_ref(), INBOX.as_ref(), STAR.as_ref(), ARCHIVE.as_ref(), TRASH.as_ref()];
    view! {
        // TODO: remove all this
        // <div>"view: "{view}</div>
        // <div>"tags: "{tags}</div>
        // <div>{loading}</div>
        // <div>{notloading}</div>
        // <div>{move || !loading()}</div>
        // <Transition /*when=notloading */ fallback=|| view! {"not loaded"}>
        //     {
        //         view! {
        //             <div id="bookmarks"><pre>{bstring}</pre></div>
        //             <button on:click=refetch>"REFETCH"</button>
        //             <button on:click=add_local>"ADD LOCAL"</button>
        //         }
        //     }
        // </Transition>
        // TODO: remove all this above
        <div class="home">
          <BookmarksProvider>
            <UrlStateProvider>
            <MenuStateProvider>
              <TopMenu />
              <section class="flex">
                <LeftMenu />
                {
                  let UrlState { view, .. } = use_context().expect("no url state resource in context!");
                  
                  move || {
                    
                    match view().as_ref() {
                      ref s if views.contains(s) =>
                          view! { <Bookmarks /> },
                      "search" =>
                          view! { <div>"search view!"</div> }.into_view(),
                      "stats" =>
                          view! { <div>"stats view!"</div> }.into_view(),
                      _ =>
                          view! { <div>"No view!"</div> }.into_view()
                    }
                  }
                }
              </section>
              // <Hotkeys />
              // <AddModal />
              <CommandModal />
              // <SettingsModal />
              // <UpdateModal />
              </MenuStateProvider>
            </UrlStateProvider>
          </BookmarksProvider>
        </div>
    }
}
