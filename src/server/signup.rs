use leptos::*;

#[server(SignUp, "/api", "Url", "signup")]
pub(crate) async fn sign_up(user: String, pass: String) -> Result<(), ServerFnError> {
    // serverside dependencies
    use actix_web::{cookie::Cookie, http::{header, header::HeaderValue}, web::Data};
    use leptos_actix::{redirect, ResponseOptions};
    use sqlx::{Executor, query, Pool, Sqlite};
    use time::OffsetDateTime;
    use crate::misc::constants::{REGEX, SESSION_DURATION};
    use crate::misc::functions::random_string;

    if !REGEX.is_match(&user) {
        logging::warn!("[registering new user] attempt to register invalid username: <{user}>");
        return Err(ServerFnError::ServerError(
            format!("username doesn't match validation pattern: {user}")
        ));
    }

    if !REGEX.is_match(&pass) {
        logging::warn!("[registering new user] attempt to register invalid password.");
        return Err(ServerFnError::ServerError("password doesn't match validation pattern.".into()));
    }

    
    let res = leptos_actix::extract(|pool: Data<Pool<Sqlite>>| async move {
        let mut txn = pool.begin().await?;
        let please_retry = Err(ServerFnError::ServerError("An error has occurred. Please retry.".into()));

        let new_user_id = uuid::Uuid::new_v4();
        let hash = bcrypt::hash(pass, 12).unwrap();

        let q = query("INSERT INTO user(id, name, email, salt, hash) VALUES(?, ?, '-', '-', ?)")
            .bind(new_user_id.to_string())
            .bind(&user)
            .bind(hash);

        let insert_user_result = txn.execute(q).await;

        match insert_user_result {
            Err(sqlx::Error::Database(e)) if e.is_unique_violation() && e.message().contains("constraint failed: user.name") => {
                logging::warn!("[registering new user] attempt to register existing username: <{user}>");
                return Err(ServerFnError::ServerError(format!("Username {user} is taken.")));
            },
            Err(e) => {
                logging::error!("[registering new user] {}", e.to_string());
                return please_retry;
            },
            _ => (),
        };

        let token = match random_string(128) {
            Err(e) => {
                logging::error!("[registering new user] problem with getrandom: {}", e.to_string());
                return please_retry;
            },
            Ok(token) => token,
        };

        let max_age = SESSION_DURATION;
        let expiry = (OffsetDateTime::now_utc() + max_age).unix_timestamp();

        let q = query("INSERT INTO session(id, user_id, token, expiry) VALUES(?, ?, ?, ?)")
            .bind(uuid::Uuid::new_v4().to_string())
            .bind(new_user_id.to_string())
            .bind(&token)
            .bind(expiry);

        match txn.execute(q).await {
            Err(sqlx::Error::Database(e)) if e.is_unique_violation() && e.message().contains("constraint failed: session.token") => {
                logging::error!("[registering new user] session token uniqueness failure: <{}>", &token);
                return please_retry;
            },
            Err(e) => {
                logging::error!("[registering new user] {}", e.to_string());
                return please_retry;
            },
            _ => {
                txn.commit().await?;
            },
        };

        let response_options = expect_context::<ResponseOptions>();
        let cookie_user = Cookie::build("user", user).max_age(max_age).path("/").finish();
        let cookie_token = Cookie::build("token", token).max_age(max_age).path("/").finish();

        response_options.insert_header(
            header::SET_COOKIE,
            HeaderValue::from_str(&cookie_user.to_string()).unwrap()
        );
        response_options.append_header(
            header::SET_COOKIE,
            HeaderValue::from_str(&cookie_token.to_string()).unwrap()
        );

        logging::log!("[registering new user] registered {new_user_id}");
        return Ok(redirect("/all")); // TODO: redirect to specific view
    }).await.unwrap();

    return res;
}
