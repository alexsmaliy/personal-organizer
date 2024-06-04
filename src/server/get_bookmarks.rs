use futures::StreamExt;
use leptos::{*, logging::log};
use crate::types::BookmarkWithTags;

#[server(GetBookmarks, "/api", "Url", "get-bookmarks")]
pub(crate) async fn get_bookmarks() -> Result<Vec<BookmarkWithTags>, ServerFnError> {
    // serverside dependencies
    use actix_web::{cookie::Cookie, http::{header, header::HeaderValue, StatusCode}, HttpRequest, web::Data};
    use leptos_actix::ResponseOptions;
    use sqlx::{Pool, Sqlite};

    let query = "
        WITH 
            q1 AS (
            SELECT
                btl.bookmark_id, t.name
            FROM
                bookmark_tag_link AS btl JOIN tag AS t
                ON btl.tag_id = t.id
            )
        SELECT
            b.*, JSON_GROUP_ARRAY(q1.name) AS tags
        FROM
            bookmark AS b JOIN q1
            ON b.id = q1.bookmark_id
            GROUP BY b.id;
    ";
    
    let res = leptos_actix::extract(move |pool: Data<Pool<Sqlite>>, req: HttpRequest| async move {
        // thread::sleep(Duration::from_millis(500)); // TODO: remove after testing
        let pool = pool.as_ref();
        // let mut result_stream = sqlx::query_as::<_, Bookmark>("SELECT * FROM bookmark").fetch(pool);
        let mut result_stream = sqlx::query_as::<_, BookmarkWithTags>(query).fetch(pool);
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
