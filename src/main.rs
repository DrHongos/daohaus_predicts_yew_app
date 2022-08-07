//use chrono::ParseError;
use yew::prelude::*;
use wasm_logger;
use yew_router::prelude::*;
use yew::html::Scope;
use ethers::prelude::*;
//use ethers_providers::{Provider, Http, Middleware};

//use ethers::types::H160;
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
    ConnectWallet(Provider<Http>),
    MessagesUser(String),
}

pub struct App {
    navbar_active: bool,
    //client: String,//Provider<Http>,
    //accounts: Vec<Address>,
}
async fn check_accounts(client: &Provider<Http>) -> Result<String, String> {
    match client.get_accounts().await {
        Ok(accs) => {
            log::info!("Accounts {:?}", &accs);
            Ok(accs[0].to_string())
        },
        Err(err) => Err(err.to_string()),
/*         {
            log::error!("Impossible to fetch accounts {:?}", err);
        }, */
    }

}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        let client = Provider::<Http>::try_from(
            "https://rpc.gnosischain.com",
        );

        ctx.link().send_future(async move {
            match client {
                Ok(preds) => Msg::ConnectWallet(preds),
                Err(err) => Msg::MessagesUser(format!("Fail {:?}", err)),
            }
        });
        Self {
            navbar_active: false,
            //client: "test".to_owned(),
            //accounts: Vec::new(),
        }
        }
    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::ToggleNavbar => {
                self.navbar_active = !self.navbar_active;
                true
            }
            Msg::ConnectWallet(client) => {
                //let accounts = client.get_accounts().await;
                //    log::info!("Accounts: {:?}", accounts);
                //  let user_account = accounts.first();
                // log::info!("Enters, provider: {:?}", &client);

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
            }
            Msg::MessagesUser(msg) => {
                log::info!("{:?}", msg);
                true
            }
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
                            {"Connected"}
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
