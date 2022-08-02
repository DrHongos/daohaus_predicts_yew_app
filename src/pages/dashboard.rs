use yew::prelude::*;

pub struct Dashboard;

impl Component for Dashboard {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <section class="hero is-danger is-bold is-large">
                <div class="hero-body">
                    <div class="container">
                        <h1 class="title">
                            { "Dashboard" }
                        </h1>
                        <h2 class="subtitle">
                            { "Supposedly to display YOUR data" }
                        </h2>
                    </div>
                </div>
            </section>
        }
    }
}
