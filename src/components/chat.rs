use serde::{Deserialize, Serialize};
use web_sys::{HtmlInputElement, KeyboardEvent};
use yew::prelude::*;
use yew_agent::{Bridge, Bridged};

use crate::services::event_bus::EventBus;
use crate::services::websocket::WebsocketService;
use crate::User;

pub enum Msg {
    HandleMsg(String),
    SubmitMessage,
    ToggleTheme,
}

#[derive(Deserialize)]
struct MessageData {
    from: String,
    message: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum MsgTypes {
    Users,
    Register,
    Message,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct WebSocketMessage {
    message_type: MsgTypes,
    data_array: Option<Vec<String>>,
    data: Option<String>,
}

pub struct Chat {
    users: Vec<String>,
    chat_input: NodeRef,
    _producer: Box<dyn Bridge<EventBus>>,
    wss: WebsocketService,
    messages: Vec<MessageData>,
    is_dark: bool,
}

impl Component for Chat {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        let (user, _) = ctx
            .link()
            .context::<User>(Callback::noop())
            .expect("context to be set");
        let wss = WebsocketService::new();
        let username = user.username.borrow().clone();

        let message = WebSocketMessage {
            message_type: MsgTypes::Register,
            data: Some(username.to_string()),
            data_array: None,
        };

        if wss
            .tx
            .clone()
            .try_send(serde_json::to_string(&message).unwrap())
            .is_ok()
        {
            log::debug!("register message sent");
        }

        Self {
            users: vec![],
            messages: vec![],
            chat_input: NodeRef::default(),
            wss,
            _producer: EventBus::bridge(ctx.link().callback(Msg::HandleMsg)),
            is_dark: false,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::HandleMsg(s) => {
                let msg: WebSocketMessage = serde_json::from_str(&s).unwrap();
                match msg.message_type {
                    MsgTypes::Users => {
                        self.users = msg.data_array.unwrap_or_default();
                        true
                    }
                    MsgTypes::Message => {
                        let message_data: MessageData =
                            serde_json::from_str(&msg.data.unwrap()).unwrap();
                        self.messages.push(message_data);
                        true
                    }
                    _ => false,
                }
            }
            Msg::SubmitMessage => {
                let input = self.chat_input.cast::<HtmlInputElement>();
                if let Some(input) = input {
                    let value = input.value();
                    if value.trim().is_empty() {
                        return false;
                    }

                    let message = WebSocketMessage {
                        message_type: MsgTypes::Message,
                        data: Some(value),
                        data_array: None,
                    };

                    if let Err(e) = self
                        .wss
                        .tx
                        .clone()
                        .try_send(serde_json::to_string(&message).unwrap())
                    {
                        log::debug!("error sending to channel: {:?}", e);
                    }
                    input.set_value("");
                };
                false
            }
            Msg::ToggleTheme => {
                self.is_dark = !self.is_dark;
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let online_count = self.users.len();
        let submit = ctx.link().callback(|_| Msg::SubmitMessage);
        let toggle_theme = ctx.link().callback(|_| Msg::ToggleTheme);
        let submit_on_enter = ctx.link().batch_callback(|e: KeyboardEvent| {
            if e.key() == "Enter" {
                Some(Msg::SubmitMessage)
            } else {
                None
            }
        });
        let theme_label = if self.is_dark { "Light" } else { "Dark" };
        let theme_class = if self.is_dark {
            "chat-shell theme-dark"
        } else {
            "chat-shell"
        };

        html! {
            <div class={theme_class}>
                <aside class="chat-sidebar">
                    <div class="chat-brand">{"YewChat"}</div>
                    <div class="chat-room">{"Studio Room"}</div>
                    <div class="chat-room-meta">{format!("Online: {}", online_count)}</div>
                    <div class="chat-room-meta">{"ws://127.0.0.1:8080"}</div>
                    <div class="chat-users-title">{"Users"}</div>
                    <ul class="chat-users">
                        if self.users.is_empty() {
                            <li class="chat-user chat-user-empty">{"Belum ada user"}</li>
                        }
                        {for self.users.iter().map(|u| html! {<li class="chat-user">{u}</li>})}
                    </ul>
                </aside>
                <main class="chat-main">
                    <header class="chat-header">
                        <div class="chat-title">{"Chat Room"}</div>
                        <div class="chat-header-actions">
                            <button class="chat-theme-toggle" onclick={toggle_theme}>{theme_label}</button>
                            <div class="chat-status">{format!("Online: {}", online_count)}</div>
                        </div>
                    </header>
                    <section class="chat-messages">
                        if self.messages.is_empty() {
                            <div class="chat-empty">{"Belum ada pesan."}</div>
                        }
                        {for self.messages.iter().map(|m| html! {
                            <div class="chat-message">
                                <div class="chat-from">{m.from.clone()}</div>
                                <div class="chat-text">{m.message.clone()}</div>
                            </div>
                        })}
                    </section>
                    <div class="chat-input">
                        <input
                            ref={self.chat_input.clone()}
                            onkeypress={submit_on_enter}
                            type="text"
                            placeholder="Ketik pesan"
                            name="message"
                            required=true
                        />
                        <button onclick={submit}>{"Kirim"}</button>
                    </div>
                </main>
            </div>
        }
    }
}
