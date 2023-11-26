use std::convert::From;
use std::ops::Not;
use leptos::*;
use wasm_bindgen::JsValue;

#[component]
pub(crate) fn LoginPage() -> impl IntoView {
    let (mode, set_mode) = create_signal(Mode::SIGNIN);
    let other_mode = move || !mode();

    let Δmode = move |_| set_mode.update(|mode| *mode = !*mode);

    view! {
        <div id="loginpage">
            <header id="intro">
                <div class="flex">
                    <h1>"Omark"</h1>
                    <button class="buttongray" prop:innerText=other_mode on:click=Δmode />
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

#[component]
fn SignUp() -> impl IntoView {
    let (user, set_user) = create_signal(String::new());
    let (pass, set_pass) = create_signal(String::new());

    let Δuser = move |ev| set_user(event_target_value(&ev));
    let Δpass = move |ev| set_pass(event_target_value(&ev));

    view! {
        <section id="login">
            <h1>"Sign Up"</h1>
            <form>
                <fieldset>
                    <label html_for="user">"User"</label>
                    <input id="user" type_="user" placeholder="HuskyEatCode" prop:value=user on:input=Δuser />
                </fieldset>
                <fieldset>
                    <label html_for="pass">"Password"</label>
                    <input id="pass" type_="pass" placeholder="nom nom" prop:value=pass on:input=Δpass />
                </fieldset>
            </form>
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