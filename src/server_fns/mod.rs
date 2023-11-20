use std::{thread, time::Duration};
use futures::StreamExt;
use leptos::{*, logging::log};

use crate::types::Bookmark;

#[server(GetBookmarks, "/api", "Url", "get-bookmarks")]
pub(super) async fn get_bookmarks() -> Result<Vec<Bookmark>, ServerFnError> {
    use actix_web::web::Data; // serverside dependency
    use sqlx::{Pool, Sqlite}; // serverside dependency
    
    leptos_actix::extract(|pool: Data<Pool<Sqlite>>| async move {
        thread::sleep(Duration::from_millis(500)); // TODO: remove after testing
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
        Ok(res)
    }).await.unwrap()
}
