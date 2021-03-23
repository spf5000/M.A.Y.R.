use yew::{Component, ComponentLink, ShouldRender, Html, html};

use crate::app_router::{AppRoute, Link};
use crate::components::coffee::coffee_summaries::CoffeeSummariesComponent;

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
                <CoffeeSummariesComponent page=0 />
            </>
        }
    }
}