use yew::{Properties, ComponentLink, Component, ShouldRender, Html, html, Bridge, Bridged};
use crate::app_router::{AppRoute, Link};
use crate::agents::coffee_store_agent::{CoffeeStoreAgent, CoffeeStoreAgentRequest, CoffeeStoreAgentResponse};
use rust_server_model::coffee_store::{ListCoffeeStoresRequest, ListCoffeeStoresResponse, CoffeeStoreSummary};
use std::default::Default;

const PAGE_SIZE: u8 = 10;
const NOT_AVAILABLE: &str = "Not Available";

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub page: u8 // hopefully there aren't more than 2560 coffee stores to page through :)
}

impl Default for Props {
    fn default() -> Self {
        Self {
            page: 0
        }
    }
}

pub enum Msg {
    DataReceived(ListCoffeeStoresResponse),
    Error(&'static str)
}

pub struct CoffeeSummariesComponent {
    page: u8,
    coffee_summary_agent: Box<dyn Bridge<CoffeeStoreAgent>>,
    coffee_summaries: Vec<CoffeeStoreSummary>,
    error: Option<String>
}

impl Component for CoffeeSummariesComponent {
    type Message = Msg;
    type Properties = Props;
    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let callback = link.callback(|response: CoffeeStoreAgentResponse| {
            match response {
                CoffeeStoreAgentResponse::ListCoffeeStoresResponse(response) => Msg::DataReceived(response),
                CoffeeStoreAgentResponse::ServerError => Msg::Error("Server Error"),
                CoffeeStoreAgentResponse::AgentError => Msg::Error("Agent Error"),
                _ => Msg::Error("Unexpected Response received!")
            }
        });
        let mut agent = CoffeeStoreAgent::bridge(callback);
        agent.send(CoffeeStoreAgentRequest::ListCoffeeStores(ListCoffeeStoresRequest {
            next_token: None,
            max_items: Some(PAGE_SIZE)
        }));
        Self {
            page: props.page,
            coffee_summary_agent: agent,
            coffee_summaries: Vec::new(),
            error: None
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::DataReceived(mut response) => {
                log::info!("Received coffee store summaries: {:?}", &response);
                self.coffee_summaries.append(&mut response.coffee_stores);
                self.error = None;
                if let Some(next_token) = response.next_token {
                    self.coffee_summary_agent.send(CoffeeStoreAgentRequest::ListCoffeeStores(ListCoffeeStoresRequest {
                        next_token: Some(next_token),
                        max_items: Some(PAGE_SIZE)
                    }));
                }
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
        // Only show the page if we've loaded enough items from the server.
        if self.coffee_summaries.len() > (self.page * PAGE_SIZE) as usize {
            let start = (self.page * PAGE_SIZE) as usize;
            let end = if start + PAGE_SIZE as usize > self.coffee_summaries.len() { self.coffee_summaries.len() } else { start + PAGE_SIZE as usize };
            render_coffee_summaries(&self.coffee_summaries[start..end])
        }
        // If there is an error, we show it
        else if self.error.is_some() {
            let error = self.error.as_ref().unwrap();
            render_error(error)
        }
        // otherwise, let the user know we're loading the data from the server.
        else {
            render_loading()
        }
    }
}

fn render_loading() -> Html {
    return html! {
        <div>
            <div class="loading">{"Loading..."}</div>
            <div><Link route=AppRoute::Home>{"Return Home!"}</Link></div>
        </div>
    }
}

fn render_error(error: &String) -> Html {
    return html! {
        <div class="components.coffee-summaries-error">
            { format!("Error! {}", error)}
        </div>
    }
}

fn render_coffee_summaries(coffee_summaries: &[CoffeeStoreSummary]) -> Html {
    return html! {
        <table class="components.coffee-summaries">
            <tr>
                <th> {"Coffee Store Name"} </th>
                <th> {"Average Rating"} </th>
                <th> {"Details Link"} </th>
            </tr>
            { coffee_summaries.iter().map(render_coffee_summary).collect::<Html>() }
        </table>
    }
}

fn render_coffee_summary(coffee_summary: &CoffeeStoreSummary) -> Html {
    let avg_rating = coffee_summary.avg_rating.map(|float| float.to_string()).unwrap_or(String::from(NOT_AVAILABLE));
    return html! {
        <tr>
            <td> {&coffee_summary.name} </td>
            <td> {avg_rating} </td>
            <td><Link route=AppRoute::CoffeeStoreDetails(coffee_summary.id.clone())> {"Details"} </Link></td>
        </tr>
    }
}
