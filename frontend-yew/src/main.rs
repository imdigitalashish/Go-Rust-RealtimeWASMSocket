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
                let cloned_ws = ws.clone();


                let cb =Closure::<dyn FnMut(_)>::new(move |e: MessageEvent| {
                    // Handle difference Text/Binary,...
                    if let Ok(abuf) = e.data().dyn_into::<js_sys::ArrayBuffer>() {
                        let array = js_sys::Uint8Array::new(&abuf);
                        let len = array.byte_length() as usize;
                        // here you can for example use Serde Deserialize decode the message
                        // for demo purposes we switch back to Blob-type and send off another binary message
                        cloned_ws.set_binary_type(web_sys::BinaryType::Blob);
                       
                    } else if let Ok(blob) = e.data().dyn_into::<web_sys::Blob>() {
                        // better alternative to juggling with FileReader is to use https://crates.io/crates/gloo-file
                        let fr = web_sys::FileReader::new().unwrap();
                        let fr_c = fr.clone();
                        // create onLoadEnd callback
                        let onloadend_cb = Closure::<dyn FnMut(_)>::new(move |_e: web_sys::ProgressEvent| {
                            let array = js_sys::Uint8Array::new(&fr_c.result().unwrap());
                            let len = array.byte_length() as usize;
                            // here you can for example use the received image/png data
                        });
                        fr.set_onloadend(Some(onloadend_cb.as_ref().unchecked_ref()));
                        fr.read_as_array_buffer(&blob).expect("blob not readable");
                        onloadend_cb.forget();
                    } else if let Ok(txt) = e.data().dyn_into::<js_sys::JsString>() {
                    } else {
                    }
                });

                ws.set_onmessage(Some(cb.as_ref().unchecked_ref()));
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