use yew::{Properties, ComponentLink, Component, ShouldRender, Html, html};
use crate::app_router::{AppRoute, Link};

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub name: String,
    pub avg_rating: Option<f32>
}

pub struct CoffeeSummary {
    // link: ComponentLink<Self>,
    props: Props
}

impl Component for CoffeeSummary {
    type Message = ();
    type Properties = Props;
    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self {
            // link,
            props
        }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        // Should only return "true" if new properties are different to
        // previously received properties.
        // This component has no properties so we will always return "false".
        if _props != self.props {
            self.props = _props.clone();
            true
        } else {
            false
        }
    }

    fn view(&self) -> Html {
        html! {
            <div class="components.coffee-summary">
                <div>{ format!("Name: {}", self.props.name) } </div>
                <div>{ format!("Average Rating: {}", match self.props.avg_rating {
                    Some(rating) => rating.to_string(),
                    None => "Unknown".to_string()
                }) } </div>
                <div><Link route=AppRoute::Home>{"Return Home!"}</Link></div>
            </div>
        }
    }
}
