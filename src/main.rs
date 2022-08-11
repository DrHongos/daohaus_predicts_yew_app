use ethers::providers::JsonRpcClient;
//use chrono::ParseError;
use yew::prelude::*;
use wasm_logger;
use yew_router::prelude::*;
use yew::html::Scope;
use wasm_bindgen::prelude::*;
use thiserror::Error;
use stdweb::{js, Value};
//use ethers::prelude::*;
use web_sys::console;
use serde::{Serialize, Deserialize};
use js_sys::Reflect;

mod components;
mod pages;
mod content;
use pages::{
    home::Home, 
    dashboard::Dashboard, 
    prediction::PredictionPage, 
    explorer::Explorer,
    page_not_found::PageNotFound,
};

#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[at("/")]
    Home,
    #[at("/dashboard")]
    Dashboard,
    #[at("/explorer")]
    Explorer,
    #[at("/prediction/:id")]
    PredictionPage {id: String},
    #[not_found]
    #[at("/404")]
    NotFound,
}

#[derive(Error, Debug)]
pub enum MetamaskError {
    #[error(transparent)]
    Json(#[from] serde_json::Error),
    #[error("no address")]
    NoAddress,
    #[error("{0}")]
    Custom(String),
}

pub enum Msg {
    ToggleNavbar,
    MessagesUser(String),
    ConnectMetamask,
    SetAddress(String),
    SetChain(String),
    SetClient(JsValue),
    //SignMessage,
}

pub struct App {
    navbar_active: bool,
    client: Option<JsValue>,
    address: Option<String>,
    chain_id: Option<String>,
    // 
}

/* #[derive(Debug, Serialize, Deserialize)]
pub struct Basic {
    pub chainId: String,
    pub isMetamask: bool,
} */

#[wasm_bindgen] //(module = "/src/jscripts/metamask.js")
extern "C" {
    #[wasm_bindgen]
    //#[derive(Deserialize, Debug)]
    pub type JsonRpcClientProxy;

///////////////////////////////////// from the console
    #[wasm_bindgen(method, structural, indexing_getter)]
    pub fn chainId(this: &JsonRpcClientProxy) -> String;
    #[wasm_bindgen(method, structural, indexing_getter)]
    pub fn isMetamask(this: &JsonRpcClientProxy) -> bool;

}
    
#[wasm_bindgen(module = "/src/jscripts/metamask.js")]
extern "C" {
    #[wasm_bindgen(js_name = "connectMetamask")]
    #[wasm_bindgen(catch)]
    pub async fn connectMetamask() -> Result<JsValue, JsValue>;

    #[wasm_bindgen(js_name = "getProvider")]
    #[wasm_bindgen(catch)]
    pub async fn getProvider() -> Result<JsValue, JsValue>;
   
    #[wasm_bindgen(js_name = "signMessage")]
    #[wasm_bindgen(catch)]
    pub async fn signMessage() -> Result<JsValue, JsValue>;
    
}

#[wasm_bindgen(module = "/src/jscripts/get-payload-script.js")]
extern "C" {
    #[wasm_bindgen(js_name = "getProviderSync")]
    pub fn get_payload() -> String;

    #[wasm_bindgen(js_name = "callEthers")]
    pub fn get_payload_later(payload_callback: JsValue);
}

impl App {
    fn get_chain_id(&self) -> String {
        if let Some(client) = &self.client {
            match Reflect::get(
                client.as_ref(), 
                &JsValue::from("chainId")
            )
                .expect("No chain connected")
                .as_string()
                {
                    Some(chain) => chain,
                    None => "Null".to_owned()
                }
        } else {
            "Not connected".to_owned()
        }
    }
    fn get_address(&self) -> Option<String> {
        if let Some(client) = &self.client {
            match Reflect::get(
                client.as_ref(), 
                &JsValue::from("selectedAddress")
            )
                .expect("No user connected")
                .as_string()
                {
                    Some(address) => Some(address),
                    None => None
                }
        } else {
            None
        }
    }
    fn has_attr(&self, attribute: String) -> Option<bool> {
        if let Some(client) = &self.client {
            let has_attr = Reflect::has(
                client.as_ref(), 
                &JsValue::from(attribute)
            )
                .expect("Not found");
            Some(has_attr)
        } else {
            None
        }
    }


}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            navbar_active: false,
            client: None,
            chain_id: None,
            address: None,
        }
        }
    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::ToggleNavbar => {
                self.navbar_active = !self.navbar_active;
                true
            }
            Msg::MessagesUser(msg) => {
                log::info!("{:?}", msg);
                true
            }
/*             Msg::CallRust => {
                log::info!("Calling to JS");
                get_payload_later(Closure::once_into_js(move |payload: String| {
                    log::info!("Returns {:?}", payload)
                }));
                false
            } */
            Msg::ConnectMetamask => {
                ctx.link().send_future(async move {
                    match getProvider().await {
                        Ok(accs) => {
                            console::log_2(&"Logging arbitrary values looks like".into(), &accs);
                            // parse
//                            let prov_parsed: Result<Basic, JsValue> = accs.into_serde().unwrap();
                                //let my_type: Basic = serde_json::from_str(&accs.as_string().unwrap()).unwrap();
                            Msg::SetClient(accs)
                        },
                        Err(err) => {
                            log::error!("Error {:?}", err);
                            Msg::MessagesUser("Err_async".to_owned())
                        },
                    }
                });
                false
            }
            Msg::SetAddress(address) => {
                log::info!("Address set to {}", &address);
                self.address = Some(address);
                true           
            }
            Msg::SetChain(chain) => {
                log::info!("Chain set to {}", &chain);
                self.chain_id = Some(chain);
                true           
            }
            Msg::SetClient(provider) => {                
//                log::info!("provider is object: {:?}", provider.is_object());
//                log::info!("provider typeof: {:?}", provider.js_typeof());               
                self.client = Some(provider);
                true   
            }
/*             Msg::SignMessage => {
                ctx.link().send_future(async move {
                    match signMessage().await {
                        Ok(_) => log::info!("Signed on Js Browser"),
                        Err(err) => log::error!("Error {:?}", err),
                    }
                });
                false
            } */
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <BrowserRouter>
                { self.view_nav(ctx.link()) }

                <main>
                    <Switch<Route> render={Switch::render(switch)} />
                </main>
                <footer class="footer">
                    <div class="content has-text-centered">
                        { "Powered by " }
                        <a href="https://yew.rs">{ "Yew" }</a>
                        { " using " }
                        <a href="https://bulma.io">{ "Bulma" }</a>
                        { " and soon " }
                        <a href="https://crates.io/crates/ethers">{ "Ethers RS" }</a>
                    </div>
                </footer>
            </BrowserRouter>
        }
    }

}
impl App {
    fn view_nav(&self, link: &Scope<Self>) -> Html {
        let Self { navbar_active, .. } = *self;

        let active_class = if !navbar_active { "is-active" } else { "" };
        //let testy =  self.provider.get_chainid();

        html! {
            <nav class="navbar is-dark" role="navigation" aria-label="main navigation">
                <div class="navbar-brand">
                    <h1 class="navbar-item is-size-3">{ "Opinologo" }</h1>

                    <button class={classes!("navbar-burger", "burger", active_class)}
                        aria-label="menu" aria-expanded="false"
                        onclick={link.callback(|_| Msg::ToggleNavbar)}
                    >
                        <span aria-hidden="true"></span>
                        <span aria-hidden="true"></span>
                        <span aria-hidden="true"></span>
                    </button>
                </div>
                <div class={classes!("navbar-menu", active_class)}>
                    <div class="navbar-start">
                        <Link<Route> classes={classes!("navbar-item")} to={Route::Home}>
                            { "Home" }
                        </Link<Route>>
                        <Link<Route> classes={classes!("navbar-item")} to={Route::Dashboard}>
                            { "Dashboard" }
                        </Link<Route>>
                        <Link<Route> classes={classes!("navbar-item")} to={Route::Explorer}>
                            { "Explorer" }
                        </Link<Route>>

                        <button 
                            class="button is-info"
                            onclick={link.callback(|_| {
/*                                 let client_parsed: JsonRpcClientProxy = self.client.clone().unwrap().into_serde().unwrap();
                                let res = client_parsed.isMetamask();                           
                                log::info!("Is metamask? {:?}", res); */
                                Msg::MessagesUser("Passed!".to_owned())
                                //Msg::SignMessage                        
                            })}
                        >                            
                            {"Test methods in client"}
                        </button>

                    </div>
                    <div class="navbar-end">
                        <div class={"navbar-item is-centered"}>
                            <div class="navbar-item is-centered has-text-white">                            
                                {"Connected to chain "}{&self.get_chain_id()}
                            </div>
                            if let Some(address) = &self.get_address() {
                                <>{address}</>
                            } else {
                                <button 
                                    class="button is-info"
                                    onclick={link.callback(|_| {
                                        Msg::ConnectMetamask
                                    })}
                                >                            
                                    {"Connect"}
                                </button>
                            }
                        </div>                    
                    </div>
                </div>
            </nav>
        }
    }

}

fn switch(routes: &Route) -> Html {
    match routes.clone() {
        Route::Dashboard => {
            html! { <Dashboard /> }
        }
        Route::Explorer => {
            html! { <Explorer /> }
        }
        Route::PredictionPage { id } => {
            html! { <PredictionPage id={id} /> }
        }
        Route::Home => {
            html! { <Home /> }
        }
        Route::NotFound => {
            html! { <PageNotFound /> }
        }
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::new(log::Level::Trace));
    yew::start_app::<App>();
}
