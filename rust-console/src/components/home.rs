use yew::{Component, ComponentLink, ShouldRender, Html, html};

use crate::app_router::{AppRoute, Link};

pub struct Home {
    // link: ComponentLink<Self>,
}

impl Component for Home {
    type Message = ();
    type Properties = ();
    fn create(_: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self {
            // link,
        }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        // Should only return "true" if new properties are different to
        // previously received properties.
        // This component has no properties so we will always return "false".
        false
    }

    fn view(&self) -> Html {
        html! {
            <>
                <header>
                    <h1> { "The Coffee Collective!" }</h1>
                </header>
                <ol>
                    <li><Link route=AppRoute::CoffeeSummary("rosolinis".to_string())>{"Rosolini's"}</Link></li>
                    <li><Link route=AppRoute::CoffeeSummary("starbucks".to_string())>{"StarBucks"}</Link></li>
                    <li><Link route=AppRoute::CoffeeSummary("joes_coffee".to_string())>{"Joe's Coffee"}</Link></li>
                </ol>
            </>
        }
    }
}