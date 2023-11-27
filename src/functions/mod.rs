use std::{thread, time::Duration};
use futures::StreamExt;
use leptos::{*, logging::log};

use crate::types::Bookmark;

#[server(GetBookmarks, "/api", "Url", "get-bookmarks")]
pub(super) async fn get_bookmarks() -> Result<Vec<Bookmark>, ServerFnError> {
    // serverside dependencies
    use actix_web::{cookie::Cookie, http::{header, header::HeaderValue, StatusCode}, HttpRequest, web::Data};
    use leptos_actix::ResponseOptions;
    use sqlx::{Pool, Sqlite};
    
    let res = leptos_actix::extract(|pool: Data<Pool<Sqlite>>, req: HttpRequest| async move {
        // thread::sleep(Duration::from_millis(500)); // TODO: remove after testing
        let pool = pool.as_ref();
        let mut result_stream = sqlx::query_as::<_, Bookmark>("SELECT * FROM bookmark").fetch(pool);
        let mut res = vec![];
        while let Some(x) = result_stream.next().await {
            match x {
                Err(e) => logging::error!("[loading bookmarks from db]: {e}"),
                Ok(c) => res.push(c),
            }
        }
        log!("loaded {} bookmarks", res.len()); // TODO: remove

        let headers = req.headers();
        let cookies = headers.get_all(header::COOKIE);
        cookies.for_each(|c| logging::log!("COOKIE: {}", c.to_str().unwrap()));

        let opts = expect_context::<ResponseOptions>();
        let cookie = Cookie::build("biscuits", "").finish();
        opts.insert_header(header::SET_COOKIE, HeaderValue::from_str(&cookie.to_string()).unwrap());
        opts.set_status(StatusCode::OK);

        Ok(res)
    }).await.unwrap();

    return res;
}
