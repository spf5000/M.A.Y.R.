use yew::{Properties, ComponentLink, Component, ShouldRender, Html, html, Bridge, Callback, Bridged};
use crate::app_router::{AppRoute, Link};
use crate::agents::coffee_summary_store::{CoffeeSummaryStore, AgentRequest, AgentResponse};
use rust_server_model::coffee_model::CoffeeSummary;

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub id: String,
}

pub enum Msg {
    DataReceived(Option<CoffeeSummary>)
}

pub struct CoffeeSummaryComponent {
    // link: ComponentLink<Self>,
    // props: Props
    coffee_summary_agent: Box<dyn Bridge<CoffeeSummaryStore>>,
    coffee_summary: Option<CoffeeSummary>
}

impl Component for CoffeeSummaryComponent {
    type Message = Msg;
    type Properties = Props;
    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let callback: Callback<AgentResponse> = link.callback(|response: AgentResponse| {
            match response {
                AgentResponse::CoffeeSummaryResponse(coffee_summary) => Msg::DataReceived(Some(coffee_summary)),
                AgentResponse::CoffeeSummaryResponseError => Msg::DataReceived(None)
            }
        });
        // let callback = link.callback(|data| Msg::DataReceived(data));
        let mut agent = CoffeeSummaryStore::bridge(callback);
        agent.send(AgentRequest::CoffeeSummaryById(props.id));
        Self {
            // link,
            // props
            coffee_summary_agent: agent,
            coffee_summary: Option::None
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::DataReceived(coffee_summary_option) => {
                self.coffee_summary = coffee_summary_option;
                true
            }
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        match &self.coffee_summary {
            Option::None => {
                return html! {
                    <div>
                        <div class="loading">{"Loading..."}</div>
                        <div><Link route=AppRoute::Home>{"Return Home!"}</Link></div>
                    </div>
                }
            },
            Option::Some(coffee_summary) => {
                return html! {
                    <div class="components.coffee-summary">
                        <div>{ format!("Name: {}", coffee_summary.name) } </div>
                        <div>{ format!("Average Rating: {}", match coffee_summary.avg_rating {
                            Some(rating) => rating.to_string(),
                            None => "Unknown".to_string()
                        }) } </div>
                        <div><Link route=AppRoute::Home>{"Return Home!"}</Link></div>
                    </div>
                }
            }
        }
    }
}
