use crate::content::PredictionCardData;
#[allow(unused_variables)]
use crate::{ Route }; 
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct PredictionCardProps {
    pub id: String,
    pub condition: Option<String>,
}

pub struct PredictionCard {
    data: PredictionCardData,
    //create all the data to be get by props
    // and fetched! i need ipfs now.. and to decode the questionId
}
impl Component for PredictionCard {
    type Message = ();
    type Properties = PredictionCardProps;

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            //data: Prediction::get(&ctx.props().id),
            data: PredictionCardData {
                    id: ctx.props().id.clone(),
                    condition: ctx.props().condition.clone(), 
                }
            }
        }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        let Self { data: _ } = self;
        html! {
            <div class="card">
                <div class="card-content">
                    <div class="media-content">
                        {self.has_condition(&self.data.condition)}
                    </div>

                    <div class="media">
                        <div class="media-left">
                            <figure class="image is-128x128">
                                <img alt="Default" src="https://gateway.pinata.cloud/ipfs/QmcwBAHktmfKU34CWxYAapGWAsdjCeHcAarRbHiPMc1cKC" />
                            </figure>
                        </div>
                        <div class="media-bottom">
                            <p class="title is-3">{ &self.data.id }</p>
                        </div>
                        
                    </div>
                </div>
                <footer class="card-footer">
                    <Link<Route> 
                        classes={classes!("card-footer-item")} 
                        to={Route::PredictionPage { id: self.data.id.clone()}}
                    >
                        { "See this prediction!" }
                    </Link<Route>>
                </footer>
            </div>
        }
    }
}

impl PredictionCard {
    fn has_condition(&self, condition: &Option<String>)-> Html {
        //let condition_id = condition.as_ref().unwrap().len();
        if let Some(condition_id) = condition {
            html! {
                <div class="icon-text">
                    <i class="fas fa-check-square"></i>
                    <div class="tile is-parent">
                        <span>{condition_id.clone()}</span>
                    </div>
                </div>
            }
        } else {
            html! {
                <div class="icon">
                    <i class="fas fa-ban"></i>
                </div>
            }
        }
    }
}