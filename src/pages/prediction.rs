use crate::{ content::Prediction}; 

use yew::prelude::*;

#[derive(Clone, Debug, Eq, PartialEq, Properties)]
pub struct Props {
    pub id: String,
}

pub struct PredictionPage {
    data: Prediction, 
}
impl Component for PredictionPage {
    type Message = ();
    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        // here fetches?
        Self {
            data: Prediction::get(&ctx.props().id),
        }
    }

    fn changed(&mut self, ctx: &Context<Self>) -> bool {
        self.data = Prediction::get(&ctx.props().id);
        true
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        let Self { data } = self;

        html! {
            <div class="section container">
                <div class="tile is-ancestor is-vertical">
                    <div class="tile is-parent">
                        <article class="tile is-child notification is-light">
                            <p class="title">{ data.id.clone() }</p>
                        </article>
                    </div>
                    <div class="tile">
                        <div class="tile is-parent is-3">
                            <article class="tile is-child notification">
                                <p class="title">{ "TODO" }</p>
                                <div class="tags">
                                    { "Get the whole Prediction data" }
                                </div>
                                <div class="tags">
                                { "from the Graph" }
                                </div>
                                <div class="tags">
                                    { "Retrieve questionId" }
                                </div>
                                <div class="tags">
                                    { "and IPFS manifesto" }
                                </div>
                                <div class="tags">
                                    { "connect wallet" }
                                </div>
                                <div class="tags">
                                    { "expose calls to smart contracts" }
                                </div>
                                <div class="tags">
                                    { "retrieve & display info about the predictions" }
                                </div>

                            </article>
                        </div>
                        <div class="tile is-parent">
                            <article class="tile is-child notification is-info">
                                <div class="content">
                                    <p class="title">{ "About me" }</p>
                                    <div class="content">
                                        {"Hacker in training"}
                                    </div>
                                    <div class="content">
                                        {data.condition.as_ref().unwrap()}
                                    </div>
                                    
                                </div>
                            </article>
                        </div>
                    </div>
                </div>
            </div>
        }
    }
}
