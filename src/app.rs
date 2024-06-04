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
