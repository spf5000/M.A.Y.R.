#![recursion_limit = "640"]
mod app_router;
mod agents;
mod components;

extern crate console_error_panic_hook;
use std::panic;

use wasm_bindgen::prelude::*;
// use yew::prelude::*;
use yew::{App, Component, ComponentLink, ShouldRender, Html, html};

use app_router::AppRouter;

struct Model {}

impl Component for Model {
    type Message = ();
    type Properties = ();
    fn create(_: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <AppRouter/>
        }
    }
}

#[wasm_bindgen(start)]
pub fn run_app() {
    // Set up logging
    wasm_logger::init(wasm_logger::Config::default());

    // Set up stack traces
    panic::set_hook(Box::new(console_error_panic_hook::hook));

    // Start the app.
    App::<Model>::new().mount_to_body();
}

// #[wasm_bindgen]
// pub fn start_worker() {
//     agents::coffee_summary_store::CoffeeSummaryStore::register();
// }
