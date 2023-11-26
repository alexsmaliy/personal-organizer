use leptos::*;
use leptos_meta::*;
use leptos_router::*;

use crate::components::{Home, LoginPage};

#[component]
pub fn Omark() -> impl IntoView {
    provide_meta_context();
    
    view! {
        <Html lang="en" />
        <Title text="Omark" />
        <Meta charset="utf-8" />
        <Meta name="viewport" content="width=device-width, initial-scale=1" />
        <Meta name="theme-color" content="#000000" />
        <Style>{include_str!("styles/reset.css")}</Style>
        <Style>{include_str!{"styles/global.css"}}</Style>
        <Stylesheet id="leptos" href="/pkg/leptos_start.css"/>
        // TODO: the line below was for the favicon -- the current one is served from /assets
        // <Link href="data:image/x-icon;base64,iVBORw0KGgoAAAANSUhEUgAAABAAAAAQEAYAAABPYyMiAAAABmJLR0T///////8JWPfcAAAACXBIWXMAAABIAAAASABGyWs+AAAAF0lEQVRIx2NgGAWjYBSMglEwCkbBSAcACBAAAeaR9cIAAAAASUVORK5CYII=" rel="icon" type_="image/x-icon" />
        <Router>
            <main>
                <Suspense fallback=move || view! {<p>"APP LOADING"</p>}> // TODO: is fallback useful?
                    <ErrorBoundary fallback=|errs| view! {
                        <ul>
                            {
                                move || errs().into_iter().map(|(_, e)| view! {
                                    <li>{e.to_string()}</li>
                                }).collect_view()
                            }
                        </ul>
                    }>
                        // <NetworkProvider> // TODO: was this finished/used in the original version?
                        // </NetworkProvider>
                        <Routes>
                            <Route path="/login" view=LoginPage />
                            <Route path="/:view/:tags?" view=Home />
                            // <Route path="/:view" view=|| view! { <div>"No view matched in router!"</div> } />
                        </Routes>
                    </ErrorBoundary>
                </Suspense>
            </main>
        </Router>
    }
}

////////////////////////////////
/// TODO: REMOVE
/// 

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

view! {
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/leptos_start.css"/>

        // sets the document title
        <Title text="Welcome to Leptos"/>

        // content for this welcome page
        <Router>
            <main>
                <ServerFun />
                <Routes>
                    <Route path="" view=HomePage/>
                    <Route path="/*any" view=NotFound/>
                </Routes>
            </main>
        </Router>
    }
}

#[server(WriteMsg, "/api", "Url", "write")]
pub async fn write_msg(message: String) -> Result<(), ServerFnError> {
    leptos_actix::extract(
        |pool: actix_web::web::Data<sqlx::Pool<sqlx::Sqlite>>| async move {
            let pool = pool.into_inner();
            let pool = pool.as_ref();
            match sqlx::query("INSERT INTO messages (message) VALUES ($1)").bind(message).execute(pool) .await {
                Ok(_row) => Ok(()),
                Err(e) => Err(ServerFnError::ServerError(e.to_string())),
            }
        }
    ).await.unwrap()
}

#[server(ReadMsg, "/api", "Url", "read")]
pub async fn read_msg() -> Result<String, ServerFnError> {
    leptos_actix::extract(
        |pool: actix_web::web::Data<sqlx::Pool<sqlx::Sqlite>>| async move {
            use sqlx::Row;
            match sqlx::query("SELECT * FROM messages").fetch_all(pool.into_inner().as_ref()) .await {
                Ok(rows) => Ok(rows.into_iter().map(|r| r.try_get::<String, _>("message").unwrap()).collect::<Vec<_>>().concat()),
                Err(e) => Err(ServerFnError::ServerError(e.to_string())),
            }
        }
    ).await.unwrap()
}

#[component]
fn ServerFun() -> impl IntoView {
    view! {
        <input type="text" name="name" on:keydown=|e| {
            logging::log!("input seen! {}", e.key());
            if e.key() == "Enter" {
                logging::log!("input submitted! {}", e.key());
                spawn_local(async move {
                    let _ = write_msg(leptos::event_target_value(&e)).await;
                })
            }
        } />
    }
}

/// Renders the home page of your application.
#[component]
fn HomePage() -> impl IntoView {
    // Creates a reactive value to update the button
    let (count, set_count) = create_signal(0);
    let on_click = move |_| set_count.update(|count| *count += 1);

    view! {
        <h1>"Welcome to Leptos!"</h1>
        <button on:input=on_click>"Click Me: " {count}</button>
    }
}

/// 404 - Not Found
#[component]
fn NotFound() -> impl IntoView {
    // set an HTTP status code 404
    // this is feature gated because it can only be done during
    // initial server-side rendering
    // if you navigate to the 404 page subsequently, the status
    // code will not be set because there is not a new HTTP request
    // to the server
    #[cfg(feature = "ssr")]
    {
        // this can be done inline because it's synchronous
        // if it were async, we'd use a server function
        let resp = expect_context::<leptos_actix::ResponseOptions>();
        resp.set_status(actix_web::http::StatusCode::NOT_FOUND);
    }

    view! {
        <h1>"Not Found"</h1>
    }
}
