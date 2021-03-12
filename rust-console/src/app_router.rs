use yew::prelude::*;
use yew_router::{Switch};
use yew_router::router::{Router};
use yew_router::components::{RouterAnchor};

use crate::components::coffee::coffee_summary::CoffeeSummaryComponent;
use crate::components::home::Home;

pub struct AppRouter {}

#[derive(Switch, Debug, Clone)]
pub enum AppRoute {
    #[to = "/coffee/{id}"]
    CoffeeSummary(String),
    #[to = "/"]
    Home,
}

pub type Link = RouterAnchor<AppRoute>;

impl Component for AppRouter {
    type Message = ();
    type Properties = ();
    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        let render_func = Router::render(|route: AppRoute| match route {
            AppRoute::Home=> html! { <Home/> },
            AppRoute::CoffeeSummary(coffee_id) => {
                html! { <CoffeeSummaryComponent id=coffee_id /> }
            },
            _ => html! { <Home/> },
        });

        html! {
            <Router<AppRoute, ()> render=render_func/>
        }
    }
}