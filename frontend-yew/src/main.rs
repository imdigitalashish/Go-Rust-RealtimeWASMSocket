use web_sys::js_sys;
use yew::prelude::*;
use web_sys::{WebSocket, MessageEvent, HtmlInputElement};
use wasm_bindgen::JsCast;


pub enum Msg {
    Connect,
    Received(String),
    Send,
    InputChanged(String),
}

pub struct WebSocketComponent {
    ws: Option<WebSocket>,
    messages: Vec<String>,
    input_value: String,
}

impl Component for WebSocketComponent {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            ws: None,
            messages: Vec::new(),
            input_value: String::new(),
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Connect => {
                let ws = WebSocket::new("ws://localhost:3000/ws").unwrap();
                let cb = ctx.link().callback(|e: MessageEvent| {
                    if let Ok(txt) = e.data().dyn_into::<js_sys::JsString>() {
                        Msg::Received(txt.as_string().unwrap())
                    } else {
                        Msg::Received("Received non-string data".to_string())
                    }
                });
                ws.set_onmessage(Some(cb.as_ref().unchecked_ref()));
                self.ws = Some(ws);
                true
            }
            Msg::Received(message) => {
                self.messages.push(message);
                true
            }
            Msg::Send => {
                if let Some(ws) = &self.ws {
                    ws.send_with_str(&self.input_value).ok();
                    self.input_value.clear();
                }
                true
            }
            Msg::InputChanged(value) => {
                self.input_value = value;
                true
            }
        }
    }
    

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div>
                <button onclick={ctx.link().callback(|_| Msg::Connect)}>{ "Connect" }</button>
                <ul>
                    { for self.messages.iter().map(|m| html! { <li>{ m }</li> }) }
                </ul>
                <input 
                    value={self.input_value.clone()}
                    oninput={ctx.link().callback(|e: InputEvent| {
                        let input: HtmlInputElement = e.target().and_then(|t| t.dyn_into::<HtmlInputElement>().ok()).unwrap();
                        Msg::InputChanged(input.value())
                    })}
                />
                <button onclick={ctx.link().callback(|_| Msg::Send)}>{ "Send" }</button>
            </div>
        }
    }
}

#[function_component]
fn App() -> Html {
    let counter = use_state(|| 0);
    let onclick = {
        let counter = counter.clone();
        move |_| {
            let value = *counter + 3;
            counter.set(value);
        }
    };

    html! {
        <div>
            <button {onclick}>{ "+1" }</button>
            <p>{ *counter }</p>
            <WebSocketComponent />
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}