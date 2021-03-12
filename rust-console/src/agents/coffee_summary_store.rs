use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use yew::worker::{AgentLink, HandlerId, Agent};
use yew::services::fetch::{FetchService, FetchTask, Response as FetchResponse, Request as FetchRequest};
use yew::format::{Nothing, Json};
use yew::agent::Context;
use yew::Callback;
use rust_server_model::coffee_model::CoffeeSummary;

#[derive(Serialize, Deserialize, Debug)]
pub enum AgentMsg{
    ResponseCallback(HandlerId, AgentResponse)
}

#[derive(Serialize, Deserialize, Debug)]
pub enum AgentRequest {
    CoffeeSummaryById(String),
}

#[derive(Serialize, Deserialize, Debug)]
pub enum AgentResponse {
    CoffeeSummaryResponse(CoffeeSummary),
    CoffeeSummaryResponseError
}

pub struct CoffeeSummaryStore {
    link: AgentLink<CoffeeSummaryStore>,
    subscribers: HashSet<HandlerId>,
    // TODO: This should be multithreaded.
    fetch_task: Option<FetchTask>,
}

impl Agent for CoffeeSummaryStore {
    // type Reach = Public<Self>;
    type Reach = Context<Self>;
    type Message = AgentMsg;
    type Input = AgentRequest;
    type Output = AgentResponse;

    fn create(link: AgentLink<Self>) -> Self {
        Self {
            link,
            subscribers: HashSet::new(),
            fetch_task: Option::None
        }
    }

    fn update(&mut self, msg: Self::Message) {
        match msg {
            AgentMsg::ResponseCallback(who, response) => {
                self.link.respond(who, response);
                self.fetch_task = Option::None;
            }
        }
    }

    fn connected(&mut self, id: HandlerId) {
        self.subscribers.insert(id);
    }

    fn handle_input(&mut self, msg: Self::Input, who: HandlerId) {
        match msg {
            AgentRequest::CoffeeSummaryById(coffee_id) => {
                log::info!("Receivied request: {}", coffee_id);
                let http_request = FetchRequest::get(format!("http://localhost:9080/coffee/{}", coffee_id))
                    .body(Nothing)
                    .unwrap();
                let http_callback: Callback<FetchResponse<Json<anyhow::Result<CoffeeSummary>>>> = self.link.callback(
                    move |response: FetchResponse<Json<anyhow::Result<CoffeeSummary>>>| {
                        let (meta, Json(data)) = response.into_parts();
                        log::info!("META: {:?}, DATA: {:?}", meta, data);
                        if meta.status.is_success() {
                            log::info!("SUCCESS!");
                            AgentMsg::ResponseCallback(who, AgentResponse::CoffeeSummaryResponse(data.unwrap()))
                        } else {
                            AgentMsg::ResponseCallback(who, AgentResponse::CoffeeSummaryResponseError)
                        }
                        // self.fetch_task = Option::None;
                    },
                );
                self.fetch_task = Option::Some(FetchService::fetch(http_request, http_callback)
                    .expect("Failed to make request to server"));
            },
        }
    }

    // fn name_of_resource() -> &'static str {
    //     "coffee_summary_store.js"
    // }
    //
    fn disconnected(&mut self, id: HandlerId) {
        self.subscribers.remove(&id);
    }
}