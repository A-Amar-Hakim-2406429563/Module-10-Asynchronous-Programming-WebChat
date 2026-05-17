use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew_router::prelude::*;

use crate::Route;
use crate::User;

#[function_component(Login)]
pub fn login() -> Html {
    let username = use_state(|| String::new());
    let user = use_context::<User>().expect("No context found.");

    let oninput = {
        let current_username = username.clone();

        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            current_username.set(input.value());
        })
    };

    let onclick = {
        let username = username.clone();
        let user = user.clone();
        Callback::from(move |_| *user.username.borrow_mut() = (*username).clone())
    };

    html! {
        <div class="login-shell">
            <div class="login-card">
                <div class="login-title">{"YewChat"}</div>
                <p class="login-subtitle">{"Enter your name to join the chat."}</p>
                <div class="login-form">
                    <input
                        {oninput}
                        class="login-input"
                        placeholder="Username"
                    />
                    <Link<Route> to={Route::Chat}>
                        <button
                            {onclick}
                            disabled={username.len() < 1}
                            class="login-button"
                        >
                            {"Join chat"}
                        </button>
                    </Link<Route>>
                </div>
            </div>
        </div>
    }
}
