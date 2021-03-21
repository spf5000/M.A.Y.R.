use yew::{Properties, ComponentLink, Component, ShouldRender, Html, html, Bridge, Callback, Bridged};
use crate::app_router::{AppRoute, Link};
use crate::agents::coffee_store_agent::{CoffeeStoreAgent, AgentRequest, AgentResponse};
use rust_server_model::coffee_store::CoffeeStoreDetails;

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub id: String,
}

pub enum Msg {
    DataReceived(CoffeeStoreDetails),
    Error(&'static str)
}

pub struct CoffeeStoreDetailsComponent {
    coffee_summary_agent: Box<dyn Bridge<CoffeeStoreAgent>>,
    coffee_store_details: Option<CoffeeStoreDetails>,
    error: Option<String>
}

impl Component for CoffeeStoreDetailsComponent {
    type Message = Msg;
    type Properties = Props;
    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let callback: Callback<AgentResponse> = link.callback(|response: AgentResponse| {
            match response {
                AgentResponse::GetCoffeeStoreResponse(details) => Msg::DataReceived(details),
                AgentResponse::ServerError => Msg::Error("Server Error"),
                AgentResponse::AgentError => Msg::Error("Agent Error"),
                _ => Msg::Error("Unexpected Response received!")
            }
        });
        let mut agent = CoffeeStoreAgent::bridge(callback);
        agent.send(AgentRequest::GetCoffeeStore(props.id));
        Self {
            coffee_summary_agent: agent,
            coffee_store_details: None,
            error: None
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::DataReceived(coffee_summary_option) => {
                self.coffee_store_details = Some(coffee_summary_option);
                self.error = None;
                true
            },
            Msg::Error(reason) => {
                self.error = Some(reason.to_string());
                true
            }
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        // if we have the details returned by the server, we show them.
        if self.coffee_store_details.is_some() {
            let details = self.coffee_store_details.as_ref().unwrap();
            let not_available = "Not Available".to_string();
            let description = details.description.as_ref().unwrap_or(&not_available);
            let rating = details.avg_rating.map(|float| float.to_string()).unwrap_or("Unknown".to_string());
            return html! {
                    <div class="components.coffee-store-details">
                        <div>{ format!("Name123: {}", details.name) } </div>
                        <div>{ format!("Description: {}", description) } </div>
                        <div>{ format!("Average Rating: {}", rating )} </div>
                        <div>{ "Some cool string" }</div>
                        <div><Link route=AppRoute::Home>{"Return Home!"}</Link></div>
                    </div>
                }
        }
        // Similarly, if there is an error, we show it
        else if self.error.is_some() {
            let error = self.error.as_ref().unwrap();
            return html! {
                    <div class="components.coffee-store-details-error">
                        { format!("Error! {}", error)}
                    </div>
            }
        }
        // otherwise, let the user know we're loading the data from the server.
        else {
            return html! {
                    <div>
                        <div class="loading">{"Loading..."}</div>
                        <div><Link route=AppRoute::Home>{"Return Home!"}</Link></div>
                    </div>
                }
        }
    }
}
