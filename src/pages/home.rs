#[allow(dead_code)]
use crate::{components::{prediction_card::PredictionCard}};
use yew::prelude::*;
use gql_client::Client;
use serde::Deserialize;

// TODO
// home could contain an explanation of the protocol
// and a list of live predictions

// to do so, create a state to store the predictions vector
// and map it for all the predictionCards
// predictionCards should have limited information (add to the hosted service the 
// retrieval of the predictions name and such.. )
// in content there's a fn get where we should get questionId -> ipfs JSON manifesto 
// then present the data in the prediction page 
// expose the functions!

// oh! and connect wallet!

// all i cannot do here, i can mix with wasm-bindgen Ts from the otherside

#[derive(Deserialize)]
pub struct PredictionData {
    id: String,
    condition: Option<String>,
}

#[derive(Deserialize)]
pub struct Data {
    predictions: Vec<PredictionData>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct FetchError {
    err: String,
}

pub enum FetchState<T> {
    NotFetching,
    Fetching,
    Success(T),
    Failed(String),
}
pub enum Msg {
    SetFetchState(FetchState<String>),
    /* GetData, */
}
// Box<dyn std::error::Error>
async fn fetch_predictions() -> Result<String, String>  {
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
            let first_item = data.predictions.first().unwrap();
            Ok(first_item.id.clone())
        },
        None => Err("Error fetching".to_string())
    }
}

pub struct Home {
//    status: FetchState<String>,
    collected: FetchState<String>, 
    //Vec<Prediction>,
}

impl Component for Home {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        // fetch predictions to make a list to display
        ctx.link().send_future(async {
            match fetch_predictions().await {
                Ok(preds) => {
                    //Msg::SetCollection(preds);
                    Msg::SetFetchState(FetchState::Success(preds))
                },
                Err(err) => Msg::SetFetchState(FetchState::Failed(err.to_string())),
            }
        });
        ctx.link()
            .send_message(Msg::SetFetchState(FetchState::Fetching));
        Self {
            collected: FetchState::NotFetching,
            //status: FetchState::NotFetching,
            //collected: Vec::new(),
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::SetFetchState(fetch_state) => {
                self.collected = fetch_state;
                true
            }
            /* Msg::GetData => {
                ctx.link().send_future(async {
                    match fetch_predictions().await {
                        Ok(preds) => {
                            //Msg::SetCollection(preds);
                            Msg::SetFetchState(FetchState::Success(preds))
                        },
                        Err(err) => Msg::SetFetchState(FetchState::Failed(err.to_string())),
                    }
                });
                ctx.link()
                    .send_message(Msg::SetFetchState(FetchState::Fetching));
                false
            } */
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div class="tile is-ancestor is-vertical">
                <div class="tile is-child hero">
                    <div class="hero-body container pb-0">
                        <h1 class="title is-1">{ "First dApp" }</h1>
                        <h2 class="subtitle">{ "...with Rust" }</h2>
                    </div>
                </div>

                <div class="tile is-child has-text-centered">
                    <figure class="image is-128x128 is-inline-block">
                        <img alt="Anything." src="https://upload.wikimedia.org/wikipedia/commons/thumb/d/d5/Rust_programming_language_black_logo.svg/1024px-Rust_programming_language_black_logo.svg.png" />
                    </figure>
                </div>

                <div class="tile is-parent container">
                    { self.explorer(ctx) }
                </div>
                <div class="tile is-parent container">
                    { self.view_info_tiles() }
                </div>

            </div>
        }
    }
}
impl Home {
    fn view_info_tiles(&self) -> Html {
        html! {
            <>
                <div class="tile is-parent">
                    <div class="tile is-child box">
                        <p class="title">{ "Why do it?" }</p>
                        <p class="subtitle">{ "Why do I think i should" }</p>

                        <div class="content">
                            {r#"
                            Rust will dominate web3 apps, its good for production, speed and
                            security! so, let's keep learning.
                            "#}
                        </div>
                    </div>
                </div>

                <div class="tile is-parent">
                    <div class="tile is-child box">
                        <p class="title">{ "When?" }</p>

                        <div class="content">
                            { "NOW!" }
                            { " There's no better time to be doing this." }
                            <br />
                            {r#"
                                Now let's fetch some data and create an explorer!
                            "#}
                            <br />
                            {r#"
                                Then a prediction page!
                            "#}
                            <br />
                            {r#"
                                a user dashboard!
                            "#}
                            <br />
                            {r#"
                                And much much more!
                            "#}
                        </div>
                    </div>
                </div>
            </>
        }
    }
    fn explorer(&self, _ctx: &Context<Self>) -> Html {
        match &self.collected {
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
            // here display the list
            FetchState::Success(id) => html! {
                <>
                    <PredictionCard id={id.clone()} />
                </>
            },
            FetchState::Failed(err) => html! {
                <>{err}</>
            },
        }

    }
}
