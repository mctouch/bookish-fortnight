#![warn(clippy::nursery, clippy::pedantic)]

use yew::prelude::*;

enum Message {}

struct Content {}

impl Component for Content {
    type Message = Message;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {}
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <>
                <main class="flex flex-grow">
                </main>
            </>
        }
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::start_app::<Content>();
}
