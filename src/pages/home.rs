#[allow(dead_code)]
use yew::prelude::*;

// all i cannot do here, i can mix with wasm-bindgen Ts from the otherside

pub struct Home {}

impl Component for Home {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
        false
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
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

}
