use leptos::*;
use leptos_router::*;

use crate::{server_fns::get_bookmarks, types::Bookmark};

#[component]
pub(crate) fn Home() -> impl IntoView {
    let params = use_params_map();
    let view = move || params.with(|p| p.get("view").cloned().unwrap());
    let tags = move || params.with(|p| p.get("tags").cloned().unwrap());
    let bookmarks_res = create_resource(||(), |_| get_bookmarks());
    let loading = bookmarks_res.loading();
    let notloading = leptos::Signal::derive(move || !loading());

    let bookmarks = move || {
        match bookmarks_res.get() {
            None => vec![],
            Some(result) => match result {
                Err(e) => {
                    logging::error!("[loading bookmarks]: {e}");
                    vec![]
                },
                Ok(b) => b,
            }
        }
    };
    let bstring = move || bookmarks().iter().map(|b| b.to_string()).collect::<Vec<String>>().join("\n");
    let refetch = move |_| bookmarks_res.refetch();
    let add_local = move |_e: leptos::ev::MouseEvent| bookmarks_res.update(|bs| match bs {
        Some(Ok(v)) => {
            let b = Bookmark { id: uuid::Uuid::default(), url: "example.com".into(), title: "example title".into(),
                                about: "example about".into(), star: true, archive: true, trash: true };
            v.push(b);
        },
        _ => {},
    });
    
    view! {
        <div>"view: "{view}</div>
        <div>"tags: "{tags}</div>
        <div>{loading}</div>
        <div>{notloading}</div>
        <div>{move || !loading()}</div>
        <Transition /*when=notloading */ fallback=|| view! {"not loaded"}>
            {
                view! {
                    <div id="bookmarks"><pre>{bstring}</pre></div>
                    <button on:click=refetch>"REFETCH"</button>
                    <button on:click=add_local>"ADD LOCAL"</button>
                }
            }
        </Transition>
    }
}
