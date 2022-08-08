//use chrono::ParseError;
use yew::prelude::*;
use wasm_logger;
use yew_router::prelude::*;
use yew::html::Scope;
use ethers::prelude::*;
use wasm_bindgen::prelude::*;

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

pub enum Msg {
    ToggleNavbar,
    //ConnectWallet(Provider<Http>),
    MessagesUser(String),
    //CallRust,
    ConnectMetamask,
    SetAddress(String),
}

pub struct App {
    navbar_active: bool,
    //client: Option<JsonRpcClient>,
    account: Option<String>,
    // 
}

#[wasm_bindgen(module = "/src/jscripts/metamask.js")]
extern "C" {
    #[wasm_bindgen(js_name = "connectMetamask")]
    #[wasm_bindgen(catch)]
    pub async fn connectMetamask() -> Result<JsValue, JsValue>;

    #[wasm_bindgen(js_name = "getProvider")]
    #[wasm_bindgen(catch)]
    pub async fn getProvider() -> Result<JsValue, JsValue>;
    
}

#[wasm_bindgen(module = "/src/jscripts/get-payload-script.js")]
extern "C" {
    #[wasm_bindgen(js_name = "getProviderSync")]
    pub fn get_payload() -> String;

    #[wasm_bindgen(js_name = "callEthers")]
    pub fn get_payload_later(payload_callback: JsValue);
}

/* async fn check_accounts(client: &Provider<Http>) -> Result<String, String> {
    match client.get_accounts().await {
        Ok(accs) => {
            log::info!("Accounts {:?}", &accs);
            Ok(accs[0].to_string())
        },
        Err(err) => Err(err.to_string()),
    }
} */

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
/*         let client = Provider::<Http>::try_from(
            "https://rpc.gnosischain.com",
        );

        ctx.link().send_future(async move {
            match client {
                Ok(preds) => Msg::ConnectWallet(preds),
                Err(err) => Msg::MessagesUser(format!("Fail {:?}", err)),
            }
        }); */
        Self {
            navbar_active: false,
            //client: None,
            account: None,
        }
        }
    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::ToggleNavbar => {
                self.navbar_active = !self.navbar_active;
                true
            }
/*             Msg::ConnectWallet(client) => {
                ctx.link().send_future(async move {
                    match check_accounts(&client).await {
                        Ok(accs) => {
                            log::info!("Accounts {:?}", accs);
                            Msg::MessagesUser("Oka".to_owned())
                        },
                        Err(err) => {
                            log::error!("Error {:?}", err);
                            Msg::MessagesUser("Nop".to_owned())
                        },
                    }
                });
                true
            } */
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
                    match connectMetamask().await {
                        Ok(accs) => {
                            //log::info!("Accounts {:?}", &accs); // here goes the type!
                            //js_sys::JsString::dyn_into(accs.as_string())
                            Msg::SetAddress(accs.into_serde::<String>().unwrap())                
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
                log::info!("Setted new state: {:?}", &address);
                self.account = Some(address);
                true           
            }
/*             Msg::SetClient(client) => {
                log::info!("Setted new client");
                self.client = Some(client);
                true           
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
                    </div>
                    <div class="navbar-end">
                        <div class={"navbar-item is-centered"}>
/*                             <button 
                                class="code-block"
                                onclick={link.callback(|_| {
                                    Msg::CallRust
                                })}
                                //value={self.payload.clone()}
                            >                            
                                    {"Connect"}
                            </button> */
                            if let Some(address) = &self.account {
                                <>{address}</>                                
                            } else {
                                <button 
                                    class="button is-info"
                                    onclick={link.callback(|_| {
                                        Msg::ConnectMetamask
                                    })}
                                    //value={self.payload.clone()}
                                >                            
                                        {"Connect"}
                                </button>
                            }

                            /*if let Some(manif) = self.data.manifesto.clone() {
                                match manif.conditionDescription {
                                    Some(res) => html!(<p>{"Connected"}</p>),
                                    None => html!()
                                }
                            } else {
                                html!()
                            } */
                        </div>                    
                    </div>
                </div>
            </nav>
        }
    }


/*     fn connect(&self) -> () {
        let provider = Provider::<Http>::try_from(
            "https://mainnet.infura.io/v3/1eb3ab620b5d481097a3cbe77c307154"
        ).expect("Could not find provider");
        if provider {
            self.provider = provider;
        } 
        ()
    } */
    
    


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
