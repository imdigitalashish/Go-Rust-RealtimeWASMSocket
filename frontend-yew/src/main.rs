use web_sys::js_sys;
use yew::prelude::*;
use web_sys::{WebSocket, MessageEvent, HtmlInputElement};
use wasm_bindgen::JsCast;
use wasm_bindgen::prelude::*;


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
                let link = ctx.link().clone();
                let onmessage_callback = Closure::<dyn FnMut(_)>::new(move |e: MessageEvent| {
                    if let Ok(txt) = e.data().dyn_into::<js_sys::JsString>() {
                        link.send_message(Msg::Received(txt.as_string().unwrap()));
                    }
                });
                ws.set_onmessage(Some(onmessage_callback.as_ref().unchecked_ref()));
                onmessage_callback.forget();
                self.ws = Some(ws);
                true
            }
            Msg::Received(message) => {
                println!("Received message: {}", message);
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


    html! {
        <div>
            <h1>{ "WebSocket Example" }</h1>
            <WebSocketComponent />
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}