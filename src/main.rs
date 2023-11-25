mod app;
mod components;
mod db;
mod errors;
mod functions;
mod types;

#[cfg(feature = "ssr")]
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    use std::time::Duration;
    use actix_files::Files;
    use actix_web::{App, HttpServer, web::Data, middleware::NormalizePath};
    use leptos::{get_configuration, logging};
    use leptos_actix::{generate_route_list, handle_server_fns, LeptosRoutes};
    use sqlx::{Executor, sqlite::{SqliteConnectOptions, SqlitePoolOptions}};
    use app::Omark;
    use db::{CONNECTION_INIT_PRAGMAS, DB_INIT_PRAGMAS, INITIALIZE_SCHEMA, SQLITE_DB_FILENAME};

    let conf = get_configuration(Some("./Cargo.toml")).await.expect("unable to find Leptos config file!");
    let addr = conf.leptos_options.site_addr;
    let routes = generate_route_list(Omark);

    let sqlite_opts = SqliteConnectOptions::new()
        .filename(SQLITE_DB_FILENAME)
        .create_if_missing(true);

    let db_pool = SqlitePoolOptions::new()
        .max_connections(5)
        .min_connections(1)
        .idle_timeout(Duration::from_secs(30))
        .max_lifetime(Duration::from_secs(600))
        .after_connect(|conn, _| Box::pin(async {
            conn.execute(CONNECTION_INIT_PRAGMAS).await?;
            Ok(())
        }))
        .connect_with(sqlite_opts)
        .await.expect("failed while initializing DB pool");

    if let Err(e) = db::run_in_txn(&db_pool, INITIALIZE_SCHEMA, "initializing schema").await {
        panic!("{e}");
    }
    if let Err(e) = db::run(&db_pool, DB_INIT_PRAGMAS, "running run-once pragmas").await {
        panic!("{e}");
    }
    logging::log!("finished initializing schema");

    let server = HttpServer::new(move || {
        let leptos_options = &conf.leptos_options;
        let site_root = &leptos_options.site_root;

        App::new()
            .wrap(NormalizePath::trim())
            .app_data(Data::new(db_pool.clone()))
            .app_data(Data::new(leptos_options.to_owned()))
            .service(Files::new("/pkg", format!("{site_root}/pkg"))) // serve JS/WASM/CSS from `pkg`
            .service(Files::new("/assets", site_root)) // serve other assets from the `assets` directory
            .service(favicon) // serve the favicon from /favicon.ico
            // TODO: replace with invidivual icon SVG's from https://fonts.google.com/icons
            .service(google_icons_woff2_font) // serve the font from /google_icons.woff2
            .route("/api/{tail:.*}", handle_server_fns())
            .leptos_routes(leptos_options.to_owned(), routes.to_owned(), Omark)
            
        //.wrap(middleware::Compress::default())
    })
    .shutdown_timeout(10 /* seconds */)
    .bind(&addr)?;

    println!("listening on http://{}", &addr);
    server.run().await
}

#[cfg(feature = "ssr")]
#[actix_web::get("favicon.ico")]
async fn favicon(opts: actix_web::web::Data<leptos::LeptosOptions>) -> impl actix_web::Responder {
    let opts = opts.into_inner();
    let ref root = opts.site_root;
    let name = format!("{root}/favicon.ico");
    actix_files::NamedFile::open_async(name).await
}

#[cfg(feature="ssr")]
#[actix_web::get("google_icons.woff2")]
async fn google_icons_woff2_font(opts: actix_web::web::Data<leptos::LeptosOptions>) -> impl actix_web::Responder {
    let opts = opts.into_inner();
    let ref root = opts.site_root;
    let name = format!("{root}/google_icons.woff2");
    actix_files::NamedFile::open_async(name).await
}

#[cfg(not(any(feature = "ssr", feature = "csr")))]
pub fn main() {
    // no client-side main function
    // unless we want this to work with e.g., Trunk for pure client-side testing
    // see lib.rs for hydration function instead
    // see optional feature `csr` instead
}

#[cfg(all(not(feature = "ssr"), feature = "csr"))]
pub fn main() {
    // a client-side main function is required for using `trunk serve`
    // prefer using `cargo leptos serve` instead
    // to run: `trunk serve --open --features csr`
    use leptos::*;
    use min::app::*;
    use wasm_bindgen::prelude::wasm_bindgen;

    console_error_panic_hook::set_once();

    leptos::mount_to_body(move |cx| {
        // note: for testing it may be preferrable to replace this with a
        // more specific component, although leptos_router should still work
        view! {cx, <App/> }
    });
}
