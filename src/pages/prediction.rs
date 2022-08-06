#![allow(non_snake_case)]


use crate::{ content::{Prediction, PredictionManifesto}}; 
use gql_client::Client;
use yew::prelude::*;
use yew_hooks::prelude::*;
use serde::{Deserialize, Serialize};
use reqwasm::http::Request;

#[derive(Serialize)]
pub struct Vars {
    pub id: String,
}
#[derive(Deserialize)]
pub struct Data {
    prediction: Prediction,
    //predictions: ChildrenWithProps<PredictionCard>,
}

#[derive(Clone, Debug, Eq, PartialEq, Properties)]
pub struct PredictionPageProps {
    pub id: String,
}

#[derive(Deserialize)]
struct ConditionFetch {
    questionId: String,
}
#[derive(Deserialize)]
struct Data2 {
    condition: ConditionFetch,
}

pub enum PredictionMsg {
    DataRetrieved(Prediction),
    QuestionRetrieved(String),
    ManifestoRetrieved(PredictionManifesto),
    Communicator(String),
}

pub struct PredictionPage {
    data: Prediction,
    errors: Option<String>,
}
async fn fetch_prediction(id: &str) -> Result<Prediction, String>  {
    let endpoint = "https://api.thegraph.com/subgraphs/name/drhongos/predictions";
    let client = Client::new(endpoint);
    let vars = Vars { id: id.to_string() };
    let pred_query = r#"
        query PredictionById($id: ID!) {
            prediction(id: $id) {
                id
                condition
                created
                collateral
                totalCollateral
                status
                timeout
                probabilitiesTotal
            }
        }    
    "#;
    let response = client.query_with_vars::<Data, Vars>(pred_query, vars).await.unwrap();
    match response {
        Some(resp) => {
            let content = resp.prediction;
            Ok(Prediction {
                id: id.to_string(),
                condition: content.condition.clone(),
                question_id: None,
                created: content.created.clone(),
                collateral: content.collateral.clone(),
                totalCollateral: content.totalCollateral.clone(),
                status: content.status.clone(),
                timeout: content.timeout.clone(),
                probabilitiesTotal: content.probabilitiesTotal.clone(),
                manifesto: None,
            })
        }
        None => Err("Error fetching".to_string())
    }
}


async fn fetch_question(id: &str) -> Result<String, String> {
    let endpoint = "https://api.thegraph.com/subgraphs/name/davidalbela/conditional-tokens-xdai";
    let client = Client::new(endpoint);
    let vars = Vars { id: id.to_string() };
    let question_query = r#"
        query GetCondition($id: ID!) {
            condition(id: $id) {
                questionId
            }
        }
    "#;
    let response = client.query_with_vars::<Data2, Vars>(question_query, vars).await.unwrap();
    match response {
        Some(resp) => Ok(resp.condition.questionId),
        None => Err("Error fetching".to_string())
    }
}

async fn fetch_manifesto(question_id: String) -> Result<PredictionManifesto, String> {
    let manifesto_url = Prediction::get_manifesto_cid(question_id);
    let mut base_url = "https://daohaus.mypinata.cloud/ipfs/".to_owned();
    base_url.push_str(&manifesto_url);
//    wasm_bindgen_futures::spawn_local(async move {
    let fetched_manifesto = Request::get(&base_url)
        .send()
        .await
        .unwrap()
        .json()
        .await
        .unwrap();
    //log::info!("fetched_manifesto {:?}", &fetched_manifesto);
    //PredictionMsg::ManifestoRetrieved(fetched_manifesto);
//        }
//    );
    Ok(fetched_manifesto)
}

impl Component for PredictionPage {
    type Message = PredictionMsg;
    type Properties = PredictionPageProps;
    
    fn create(ctx: &Context<Self>) -> Self {
        // create a msg to set state after the searches    
        let id = ctx.props().id.clone();
        let id2 = ctx.props().id.clone(); // why why why...
        log::info!("calling prediction: {:?}", &id);
        ctx.link().send_future(async move {
            match fetch_prediction(&id).await {
                Ok(preds) => PredictionMsg::DataRetrieved(preds),
                Err(err) => PredictionMsg::Communicator(err),
            }
        });
        Self { data: Prediction::empty(id2), errors: None }
    }
    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            PredictionMsg::DataRetrieved(preds) => {
                let condition = preds.condition.clone();
                self.data = Prediction::set(preds);
                if let Some(condition_id) = condition {
                    //log::info!("looking for: {:?}", &condition_id);
                    ctx.link().send_future(async move {
                        match fetch_question(&condition_id).await {
                            Ok(res) => PredictionMsg::QuestionRetrieved(res), // call the manifesto fetch!
                            Err(err) => PredictionMsg::Communicator(err),
                        }
                    });        
                } else {
                    let msg = "Not initialized".to_string();
                    log::error!("Setting error: {:?}", &msg);
                    self.errors = Some(msg);
                }
                true
            }
            PredictionMsg::Communicator(msg) => {
                log::error!("Setting happened: {:?}", &msg);
                self.errors = Some(msg);
                true
            }
            PredictionMsg::QuestionRetrieved(question_id) => {
                self.data.question_id = Some(question_id.clone());
                ctx.link().send_future(async move {
                    match fetch_manifesto(question_id.clone()).await {
                        Ok(res) => {                            
                            PredictionMsg::ManifestoRetrieved(res)
                        },
                        Err(err) => PredictionMsg::Communicator(err),
                    }
                });        
                true
            }
            PredictionMsg::ManifestoRetrieved(manifesto) => {
                //log::info!("Setted manifesto {:?}", &manifesto);
                self.data.manifesto = Some(PredictionManifesto::parse(manifesto));
                true
            }
            /*          
            Msg::FillPredictions(data) => {
                // handle internal state for filling a tx
                self.status = FetchState::Success;
                self.collected = Some(data);
                true
            }
  */
       }
    }
    fn view(&self, _ctx: &Context<Self>) -> Html {
        let Self { data, errors: _ } = self;
        html! {
            <div class="section container">
                <div class="tile is-ancestor is-vertical">
                    <div class="tile is-parent">
                        <article class="tile is-child notification is-light">                        
                            {self.has_title(self.data.manifesto.clone())}
                            <HelloAddress title={"ID: "} data={data.id.clone()} />
                            <br />
                            <p class="subtitle">{"Created: "}{ data.created.clone() }</p>
                            {self.has_oracle(self.data.manifesto.clone())}
                        </article>
                    </div>
                    <div class="tile">
                        <div class="tile is-parent is-3">
                            <article class="tile is-child notification">
                                <p class="title">{ "TODO" }</p>
                                <div class="tags">
                                    { "create functional components for data (and inputs)" }                                    
                                </div>
                                <div class="tags">
                                    { "connect wallet" }
                                </div>
                                <div class="tags">
                                    { "expose calls to smart contracts" }
                                </div>
                                <div class="tags">
                                    { "retrieve & display info about the predictions (thegraph)" }
                                </div>
                                <div class="tags">
                                    { "read contracts, listen events, etc" }
                                </div>

                            </article>
                        </div>
                        <div class="tile is-parent">
                            {self.has_attribute(&self.data.condition)}
                            {self.has_errors(&self.errors)}
                        </div>
                    </div>
                </div>
            </div>
        }
    }
}

impl PredictionPage {
    fn has_title(&self, manifesto: Option<PredictionManifesto>) -> Html {
        match manifesto {
            Some(manifestoData) => {
                match manifestoData.title {
                    None => html! {"no title"},
                    Some(title) => html! { <p class="title">{title}</p> },
                }
            },
            None => html! {"error"}
        }
    }
    fn has_oracle(&self, manifesto: Option<PredictionManifesto>) -> Html {
        match manifesto {
            Some(manifestoData) => {
                match manifestoData.oracle {
                    None => html! {"not recognized"},
                    Some(oracle) => html! {
                        <>
                            <HelloAddress title={"Oracle: "} data={oracle} />
                        </>
                    },
                }
            },
            None => html! {"error"}
        }
    }    
    fn has_attribute(&self, condition: &Option<String>)-> Html {
        //let condition_id = condition.as_ref().unwrap().len();
        if let Some(res) = condition {
            html! {
                <article class="tile is-child notification is-info">
                    <div class="content">
                        <p class="title">{ "About it" }</p>
                        <div class="content">
                        
                            {
                                if let Some(manif) = self.data.manifesto.clone() {
                                    match manif.conditionDescription {
                                        Some(res) => html!(<p>{"Condition description: "}{res}</p>),
                                        None => html!()
                                    }
                                } else {
                                    html!()
                                }
                            }

                            {
                                if let Some(manif) = self.data.manifesto.clone() {
                                    match manif.description {
                                        Some(desc) => html!(<p>{"Event description: "}{desc}</p>),
                                        None => html!()
                                    }
                                } else {
                                    html!()
                                }
                            }
                            <p class="title">{ "Condition ID" }</p>
                            <p class="subtitle">{res.clone()}</p>                            
                            {
                                if let Some(manif) = self.data.manifesto.clone() {
                                    match manif.minionToken {
                                        Some(res) => html!(<HelloAddress title={"Token: "} data={res} />),
                                        None => html!()
                                    }
                                } else {
                                    html!()
                                }
                            }
                            {
                                if let Some(manif) = self.data.manifesto.clone() {
                                    html!(<h3>{"Outcomes"}</h3>);
                                    match manif.outcomes {
                                        Some(res) => {
                                            res.iter().map(|outcome| html! {
                                                <p>{format!("{}", outcome)}</p>
                                            }).collect()
                                        }
                                                
                                        None => html!()
                                    }
                                } else {
                                    html!()
                                }

                            }                            
                            //
                            //
                            //outcomes (loop)

                        </div>
                    </div>
                </article>     
            }
        } else {
            html! {
                <>
                </>
            }
        }
    }
    fn has_errors(&self, errors: &Option<String>)-> Html {
        //let condition_id = condition.as_ref().unwrap().len();
        if let Some(err) = errors {
            html! {
                <article class="tile is-child notification is-danger is-bold">
                    <div class="content">
                        <p class="title">{ "Errors" }</p>
                        <div class="content">
                            {"Something happened"}
                        </div>
                        <div class="content">
                            <h1 class="title">{ "Errors" }</h1>
                            <h2 class="subtitle">{err.clone()}</h2>                            
                        </div>
                    </div>
                </article>     
            }
        } else {
            html! {
                <>
                    //{"All is good and well"}
                </>
            }
        }
    }

}



#[derive(Properties, PartialEq)]
pub struct UiAddressProps {
    pub title: Option<String>,
    pub data: String, // string?? maybe find a better suited type
}


#[function_component(HelloAddress)]
fn ui_address(props: &UiAddressProps) -> Html {
    let open = use_bool_toggle(false);
    let clipboard = use_clipboard();

    let onclick = {
        let open = open.clone();
        Callback::from(move |_| open.toggle())
    };
    let onclick_write_text = {
        let clipboard = clipboard.clone();
        let data_to_copy = props.data.trim().to_owned();
        Callback::from(move |_| {
            clipboard.write_text((*data_to_copy).to_string());
            //DialogService::alert((*data_to_copy).to_string()+" copied to clipboard");
        })
    };
    html! {
        <div class="card">
            <header class="card-header">
                if let Some(title) = &props.title {
                    <p class="card-header-title">
                        {title}
                    </p>
                }                   
                <p class="card-header-title is-centered">
                    {props.data.clone()}
                </p>
                <button {onclick} class="card-header-icon" aria-label="more options">
                    <span class="icon">
                        <i class="fas fa-angle-down" aria-hidden="true"></i>
                    </span>
                </button>
            </header>
            if *open {
                <footer class="card-footer">
                    <button onclick={onclick_write_text} class="card-footer-item is-link">
                        <a>
                            {"Copy"}
                        </a>
                    </button>
                    <button class="card-footer-item is-link">
                        <a                        
                            href={format!("https://blockscout.com/xdai/mainnet/address/{}",props.data.clone())} 
                            target="_blank"
                        >
                            {"Explorer"}                    
                        </a>
                    </button>
                </footer>
            } 
            </div>
    }
}