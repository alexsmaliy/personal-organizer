use std::convert::From;
use std::ops::Not;
use lazy_static::lazy_static;
use leptos::*;
use leptos_router::{ActionForm, Form, FromFormData};
use regex::Regex;
use wasm_bindgen::JsValue;
use web_sys::SubmitEvent;

#[server(SignUp, "/api", "Url", "signup")]
pub(super) async fn sign_up(user: String, pass: String) -> Result<(), ServerFnError> {
    // serverside dependencies
    use actix_web::{cookie::Cookie, http::{header, header::HeaderValue, StatusCode}, HttpRequest, web::{Data, Form}};
    use leptos_actix::ResponseOptions;
    use sqlx::{Pool, Row, Sqlite};

    std::thread::sleep(std::time::Duration::from_millis(500));

    if !REGEX.is_match(&user) {
        return Err(ServerFnError::ServerError(
            format!("username doesn't match validation pattern: {user}")
        ));
    }

    if !REGEX.is_match(&pass) {
        return Err(ServerFnError::ServerError(
            format!("password doesn't match validation pattern: {pass}")
        ));
    }

    let res = leptos_actix::extract(|pool: Data<Pool<Sqlite>>| async move {
        let pool = pool.as_ref();

        match sqlx::query("SELECT COUNT(*) AS count FROM user WHERE name = ?").bind(&user).fetch_one(pool).await {
            Err(err) => {
                logging::error!("{}", err.to_string());
                return Err(ServerFnError::ServerError("Failed to connect to DB!".into()));
            },
            Ok(row) => match row.try_get::<i64, _>("count") {
                Err(err) => {
                    logging::error!("{}", err.to_string());
                    return Err(ServerFnError::ServerError("Error querying DB!".into()));
                },
                Ok(count) => {
                    logging::log!("USER COUNT: {count}");
                    if count > 0 {
                        return Err(ServerFnError::ServerError("Username is taken!".into()));
                    }
                },
            },
        };
        
        return Ok(());
    }).await.unwrap();

    return res;
}

#[component]
pub(crate) fn LoginPage() -> impl IntoView {
    let (mode, set_mode) = create_signal(Mode::SIGNIN);

    let opposite_mode = move || !mode();
    
    let Δmode = move |_| set_mode.update(|mode| *mode = !*mode);

    view! {
        <div id="loginpage">
            <header id="intro">
                <div class="flex">
                    <h1>"Omark"</h1>
                    <button class="buttongray" prop:innerText=opposite_mode on:click=Δmode />
                </div>
                <p>"A fast & simple bookmarking site."</p>
            </header>
            <div id="main">
                <section id="features">
                    <h2>"Features"</h2>
                    <ol>
                        <li>"Shareable."</li>
                        <li>"Easy export."</li>
                        <li>"No tracking code."</li>
                        <li>"Husky would approve."</li>
                        <li>"Mostly local for speed."</li>
                        <li>"Hotkey-driven because that's how I roll."</li>
                        <li>"Custom views & bookmark classification (Coming)."</li>
                        <li>"Google-class support — yell on HN to get my attention."</li>
                    </ol>
                </section>
                { move || View::from(mode()) }
            </div>
            <footer>
                <div class="horizontal-line" />
                <p>"Copyright © 2023 Nom Nom Husky Enterprises"</p>
            </footer>
        </div>
    }
}

#[component]
fn SignIn() -> impl IntoView {
    "Sign in"
}

lazy_static! {
    // 1 x (letter | number | punct | symbol | CJK | emoji)
    // 2+ x (as above, but also any space)
    // 1 x (letter | number | punct | symbol | CJK | emoji)
    static ref REGEX: Regex = Regex::new("\
        ^\
            [\
                \\p{Alphabetic}\
                \\p{Decimal_Number}\
                \\p{Punctuation}\
                \\p{Symbol}\
                \\p{Ideographic}\
                \\p{Emoji}\
            ]\
            [\
                \\p{Alphabetic}\
                \\p{Decimal_Number}\
                \\p{Punctuation}\
                \\p{Symbol}\
                \\p{Space_Separator}\
                \\p{Ideographic}\
                \\p{Emoji}\
            ]\
            {2,}\
            [\
                \\p{Alphabetic}\
                \\p{Decimal_Number}\
                \\p{Punctuation}\
                \\p{Symbol}\
                \\p{Ideographic}\
                \\p{Emoji}\
            ]\
        $\
    ").expect("validation regex didn't compile!");
}

#[component]
fn SignUp() -> impl IntoView {
    let (user, set_user) = create_signal(String::new());
    let (user_err, set_user_err) = create_signal(false);
    let (pass, set_pass) = create_signal(String::new());
    let (pass_err, set_pass_err) = create_signal(false);

    let Δuser = move |ev| {
        let user = event_target_value(&ev);
        set_user_err(!REGEX.is_match(user.as_ref()));
        set_user(user);
    };

    let Δpass = move |ev| {
        let pass = event_target_value(&ev);
        set_pass_err(!REGEX.is_match(pass.as_ref()));
        set_pass(pass);
    };

    let signup = create_server_action::<SignUp>();
    let signup_pending = signup.pending();
    let signup_result = signup.value();

    let validate_form = move |ev: SubmitEvent| {
        if user_err() || pass_err() {
            ev.prevent_default();
        }
    };

    view! {
        <section id="login">
            <h1>"Sign Up"</h1>
            <ActionForm action=signup on:submit=validate_form>
                <fieldset>
                    <label for="user">"User"</label>
                    <input
                        id="user"
                        name="user"
                        type="text"
                        placeholder="HuskyEatCode"
                        prop:value=user
                        on:input=Δuser
                        class:invalid=user_err
                        required
                    />
                </fieldset>
                <fieldset>
                    <label for="pass">"Password"</label>
                    <input
                        id="pass"
                        name="pass"
                        type="password"
                        placeholder="nom nom"
                        prop:value=pass
                        on:input=Δpass
                        class:invalid=pass_err
                        required
                    />
                </fieldset>
                <button
                    type="submit"
                    prop:innerText=move || if signup_pending() { "Loading" } else { "Sign Up" }
                    prop:disabled=signup_pending
                />
                <Show when=move || !signup_pending() >
                    {
                        move || match signup_result() {
                            Some(Err(ServerFnError::ServerError(err))) => view! {<p>{err}</p>}.into_view(),
                            _ => ().into_view(),
                        }
                    }
                </Show>
            </ActionForm>
        </section>
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Mode {
    SIGNUP, SIGNIN,
}

impl Not for Mode {
    type Output = Self;

    fn not(self) -> Self::Output {
        match self {
            Self::SIGNUP => Self::SIGNIN,
            Self::SIGNIN => Self::SIGNUP,
        }
    }
}

impl From<Mode> for JsValue {
    fn from(mode: Mode) -> Self {
        JsValue::from_str(
            if mode == Mode::SIGNIN {
                "Sign In"
            } else {
                "Sign Up"
            }
        )
    }
}

impl From<Mode> for View {
    fn from(mode: Mode) -> Self {
        if mode == Mode::SIGNIN {
            SignIn().into_view()
        } else {
            SignUp().into_view()
        }
    }
}