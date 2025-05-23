use serde::{Deserialize, Serialize};
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew_agent::{Bridge, Bridged};

use crate::services::event_bus::EventBus;
use crate::{services::websocket::WebsocketService, User};

pub enum Msg {
    HandleMsg(String),
    SubmitMessage,
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

#[derive(Clone)]
struct UserProfile {
    name: String,
    avatar: String,
}

pub struct Chat {
    users: Vec<UserProfile>,
    chat_input: NodeRef,
    _producer: Box<dyn Bridge<EventBus>>,
    wss: WebsocketService,
    messages: Vec<MessageData>,
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

        if let Ok(_) = wss
            .tx
            .clone()
            .try_send(serde_json::to_string(&message).unwrap())
        {
            log::debug!("message sent successfully");
        }

        Self {
            users: vec![],
            messages: vec![],
            chat_input: NodeRef::default(),
            wss,
            _producer: EventBus::bridge(ctx.link().callback(Msg::HandleMsg)),
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::HandleMsg(s) => {
                let msg: WebSocketMessage = serde_json::from_str(&s).unwrap();
                match msg.message_type {
                    MsgTypes::Users => {
                        let users_from_message = msg.data_array.unwrap_or_default();
                        self.users = users_from_message
                            .iter()
                            .map(|u| UserProfile {
                                name: u.into(),
                                avatar: format!(
                                    "https://avatars.dicebear.com/api/adventurer-neutral/{}.svg",
                                    u
                                )
                                .into(),
                            })
                            .collect();
                        return true;
                    }
                    MsgTypes::Message => {
                        let message_data: MessageData =
                            serde_json::from_str(&msg.data.unwrap()).unwrap();
                        self.messages.push(message_data);
                        return true;
                    }
                    _ => {
                        return false;
                    }
                }
            }
            Msg::SubmitMessage => {
                let input = self.chat_input.cast::<HtmlInputElement>();
                if let Some(input) = input {
                    let message = WebSocketMessage {
                        message_type: MsgTypes::Message,
                        data: Some(input.value()),
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
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let submit = ctx.link().callback(|_| Msg::SubmitMessage);

        html! {
            <div class="flex w-screen font-sans text-white bg-black">
                <div class="flex-none w-60 h-screen bg-gradient-to-b from-indigo-950 via-black to-gray-900 border-r border-indigo-800">
                    <div class="text-lg px-4 py-3 text-indigo-300 font-bold tracking-wider border-b border-indigo-700 flex items-center gap-2">
                        {"Users"}
                    </div>
                    {
                        self.users.clone().iter().map(|u| {
                            html!{
                                <div class="flex m-3 bg-gradient-to-r from-indigo-900 to-black rounded-xl p-2 border border-indigo-700 hover:brightness-110 transition">
                                    <img class="w-10 h-10 rounded-full border border-indigo-400" src={u.avatar.clone()} alt="avatar"/>
                                    <div class="flex-grow px-3 pt-1">
                                        <div class="text-sm font-semibold text-indigo-100">{u.name.clone()}</div>
                                        <div class="text-xs text-indigo-400">{"Hi there!"}</div>
                                    </div>
                                </div>
                            }
                        }).collect::<Html>()
                    }
                </div>

                <div class="grow h-screen flex flex-col bg-gradient-to-br from-black via-gray-950 to-indigo-950">
                    <div class="w-full h-16 bg-indigo-950 border-b border-indigo-700 flex items-center px-6 text-indigo-300 font-bold text-lg tracking-wide">
                        {"💬 YewChat"}
                    </div>
 
                    <div class="w-full grow overflow-auto px-6 py-6 bg-gray-950 bg-opacity-90">
                        {
                            self.messages.iter().map(|m| {
                                let user = self.users.iter().find(|u| u.name == m.from).unwrap();
                                let is_self = user.name == "your_username";
        
                                html!{
                                    <div class={
                                        format!("flex items-start max-w-[60%] m-4 px-5 py-3 gap-3 rounded-2xl border shadow-md transition-all {}",
                                            if is_self {
                                                "bg-indigo-800 bg-opacity-50 border-indigo-400 ml-auto"
                                            } else {
                                                "bg-gray-800 bg-opacity-50 border-indigo-700"
                                            }
                                        )
                                    }>
                                        <img class="w-8 h-8 rounded-full border border-indigo-400" src={user.avatar.clone()} alt="avatar"/>
                                        <div>
                                            <div class="text-sm font-semibold text-indigo-100">{m.from.clone()}</div>
                                            <div class="text-sm text-indigo-300 mt-1 break-words">
                                                if m.message.ends_with(".gif") {
                                                    <img class="mt-2 rounded-md" src={m.message.clone()}/>
                                                } else {
                                                    {m.message.clone()}
                                                }
                                            </div>
                                        </div>
                                    </div>
                                }
                            }).collect::<Html>()
                        }
                    </div>
        
                    <div class="w-full h-20 flex px-6 items-center border-t border-indigo-800 bg-gradient-to-r from-indigo-900 via-black to-gray-900">
                        <input 
                            ref={self.chat_input.clone()} 
                            type="text" 
                            placeholder="Type a message..." 
                            class="flex-grow py-3 px-5 mr-3 bg-black bg-opacity-40 text-white placeholder-indigo-300 rounded-full border border-indigo-600 outline-none focus:ring-2 focus:ring-indigo-500"
                            name="message" 
                            required=true 
                        />
                        <button 
                            onclick={submit} 
                            class="p-3 bg-gradient-to-tr from-indigo-500 to-violet-600 w-12 h-12 rounded-full flex justify-center items-center shadow-lg hover:scale-105 transition-transform"
                        >
                            <svg viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg" class="fill-white">
                                <path d="M0 0h24v24H0z" fill="none"/>
                                <path d="M2.01 21L23 12 2.01 3 2 10l15 2-15 2z"/>
                            </svg>
                        </button>
                    </div>
                </div>
            </div>
        }
    }
}