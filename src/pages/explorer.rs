#[allow(dead_code)]
use crate::{components::{prediction_card::PredictionCard}};
use crate::{ content::PredictionCardData}; 

use yew::prelude::*;
use gql_client::Client;
use serde::Deserialize;


#[derive(Deserialize)]
pub struct Data {
    predictions: Vec<PredictionCardData>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct FetchError {
    err: String,
}

pub enum FetchState<String> {
    NotFetching,
    Fetching,
    Success,
    Failed(String),
}
pub enum Msg {
    FillPredictions(Data),
    SetFetchState(FetchState<String>),
}
// Box<dyn std::error::Error> // Handle Errors!
async fn fetch_predictions() -> Result<Data, String>  {
    let endpoint = "https://api.thegraph.com/subgraphs/name/drhongos/predictions";
    let client = Client::new(endpoint);
    let query = r#"
        query AllPredictions {
            predictions(first: 10) {
                id
                condition
            }
        }    
    "#;
    let response = client.query::<Data>(query).await.unwrap();
    match response {
        Some(data) => {
            Ok(data)
        },
        None => Err("Error fetching".to_string())
    }
}

pub struct Explorer {
    status: FetchState<String>,
    collected: Option<Data>,
}

impl Component for Explorer {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        ctx.link().send_future(async {
            match fetch_predictions().await {
                Ok(preds) => {
                    Msg::FillPredictions(preds)
                },
                Err(err) => Msg::SetFetchState(FetchState::Failed(err.to_string())),
            }
        });
        ctx.link()
            .send_message(Msg::SetFetchState(FetchState::Fetching));
        Self {
            status: FetchState::NotFetching,
            collected: None,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::SetFetchState(fetch_state) => {
                self.status = fetch_state;
                true
            }
            Msg::FillPredictions(data) => {                
                self.status = FetchState::Success;
                self.collected = Some(data);
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div class="tile is-ancestor is-vertical">
                <div class="tile is-child hero">
                    <div class="hero-body container pb-0">
                        <h1 class="title is-1">{ "Explore predictions" }</h1>
                    </div>
                </div>
                <div class="tile is-parent container">
                    { self.explorer(ctx) }
                </div>

            </div>
        }
    }
}
impl Explorer {
    fn explorer(&self, _ctx: &Context<Self>) -> Html {
        match &self.status {
            FetchState::NotFetching => html! {
                <>
                    {"Not fetching"}
                    /* <button onclick={ctx.link().callback(|_| Msg::GetData)}>
                        { "Explorer" }
                    </button> */
                </>
            },
            FetchState::Fetching => html! {
                <>{"Fetching"}</>
            },
            FetchState::Success => {
                let predictions = self.collected.as_ref().unwrap().predictions.iter().map(|data| {
                    html! {
                        <div class="tile is-parent">
                            <div class="tile is-child">
                                <PredictionCard 
                                    id={data.id.clone()}
                                    condition={data.condition.clone()}
                                />
                            </div>
                        </div>
                    }
                });
                html! {
                    <div class="tile is-ancestor">
                        { for predictions }
                    </div>
                }
            }
            FetchState::Failed(err) => html! {
                <>{err}</>
            },
        }

    }
}
