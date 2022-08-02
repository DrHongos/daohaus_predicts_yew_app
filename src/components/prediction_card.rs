#[allow(unused_variables)]
use crate::{ content::Prediction, Route }; 
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct Props {
    pub id: String,
}

pub struct PredictionCard {
    data: Prediction,
    //create all the data to be get by props
    // and fetched! i need ipfs now.. and to decode the questionId
}
impl Component for PredictionCard {
    type Message = ();
    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
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
            <div class="card">
                <div class="card-content">
                    <div class="media">
                        <div class="media-left">
                            <figure class="image is-128x128">
                                <img alt="Default" src="https://gateway.pinata.cloud/ipfs/QmcwBAHktmfKU34CWxYAapGWAsdjCeHcAarRbHiPMc1cKC" />
                            </figure>
                        </div>
                        <div class="media-content">
                            <p class="title is-3">{ &self.data.id }</p>
                        </div>
                    </div>
                </div>
                <footer class="card-footer">
                    <Link<Route> classes={classes!("card-footer-item")} to={Route::PredictionPage { id: self.data.id.clone() }}>
                        { "See this prediction!" }
                    </Link<Route>>
                </footer>
            </div>
        }
    }
}
