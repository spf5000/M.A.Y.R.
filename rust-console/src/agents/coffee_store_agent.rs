use serde::{Deserialize, Serialize};
use std::collections::{HashSet, HashMap};
use yew::worker::{AgentLink, HandlerId, Agent};
use yew::services::fetch::{FetchService, FetchTask, Response as FetchResponse, Request as FetchRequest};
use yew::format::{Text};
use yew::agent::Context;
use yew::Callback;
use rust_server_model::coffee_store::{CoffeeStoreManifest, CoffeeStoreDetails, GetCoffeeStoreRequest, CreateCoffeeStoreRequest, ListCoffeeStoresRequest, ListCoffeeStoresResponse, CreateCoffeeStoreResponse, GetCoffeeStoreResponse};
use crate::error::SimpleError;

const SERVER_URL_BASE: &'static str = "http://localhost:9080/coffee/";

#[derive(Serialize, Deserialize, Debug)]
pub enum AgentMsg{
    ResponseCallback(HandlerId, CoffeeStoreAgentResponse)
}

/* These "should" have their own "internal"/client side structures to decouple the agent APIs from
 * the backend. However, they're currently exactly the same with no clear UI specific abstractions.
 */
#[derive(Serialize, Deserialize, Debug)]
pub enum CoffeeStoreAgentRequest {
    GetCoffeeStore(String),
    ListCoffeeStores(ListCoffeeStoresRequest),
    CreateCoffeeStore(CoffeeStoreManifest),
}

#[derive(Serialize, Deserialize, Debug)]
pub enum CoffeeStoreAgentResponse {
    GetCoffeeStoreResponse(CoffeeStoreDetails),
    ListCoffeeStoresResponse(ListCoffeeStoresResponse),
    CreateCoffeeStoreResponse(CoffeeStoreDetails),
    ServerError,
    AgentError
}

pub struct CoffeeStoreAgent {
    link: AgentLink<CoffeeStoreAgent>,
    subscribers: HashSet<HandlerId>,
    // TODO: This should be multithreaded.
    fetch_tasks: HashMap<HandlerId, FetchTask>,
}

impl CoffeeStoreAgent {
    fn get_coffee_store(&mut self, who: HandlerId, coffee_store_id: String) -> anyhow::Result<()> {
        log::info!("Getting Coffee Store by id: {}", coffee_store_id);
        let http_request = create_request("get", &GetCoffeeStoreRequest {
            coffee_store_id
        })?;
        let http_callback: Callback<FetchResponse<Text>> = self.link.callback(
            move |response: FetchResponse<Text>| {
                match parse_response::<GetCoffeeStoreResponse>(&response) {
                    Ok(server_response) => AgentMsg::ResponseCallback(who, CoffeeStoreAgentResponse::GetCoffeeStoreResponse(server_response.coffee_store_details)),
                    Err(_) => AgentMsg::ResponseCallback(who, CoffeeStoreAgentResponse::ServerError)
                }
            },
        );
        let fetch_task = FetchService::fetch(http_request, http_callback)?;
        self.fetch_tasks.insert(who, fetch_task);
        Ok(())
    }

    fn create_coffee_store(&mut self, who: HandlerId, manifest: CoffeeStoreManifest) -> anyhow::Result<()> {
        log::info!("Creating Coffee Store: {:?}", manifest);
        let http_request = create_request("create", &CreateCoffeeStoreRequest {
            coffee_store: manifest
        })?;
        let http_callback: Callback<FetchResponse<Text>> = self.link.callback(
            move |response: FetchResponse<Text>| {
                match parse_response::<CreateCoffeeStoreResponse>(&response) {
                    Ok(server_response) => AgentMsg::ResponseCallback(who, CoffeeStoreAgentResponse::CreateCoffeeStoreResponse(server_response.coffee_store_details)),
                    Err(_) => AgentMsg::ResponseCallback(who, CoffeeStoreAgentResponse::ServerError)
                }
            },
        );
        let fetch_task = FetchService::fetch(http_request, http_callback)?;
        self.fetch_tasks.insert(who, fetch_task);
        Ok(())
    }

    fn list_coffee_stores(&mut self, who: HandlerId, next_token: Option<String>, page_size: Option<u8>) -> anyhow::Result<()> {
        log::info!("Listing Coffee Stores");
        let http_request = create_request("list", &ListCoffeeStoresRequest {
            max_items: page_size,
            next_token
        })?;
        let http_callback: Callback<FetchResponse<Text>> = self.link.callback(
            move |response: FetchResponse<Text>| {
                match parse_response::<ListCoffeeStoresResponse>(&response) {
                    Ok(server_response) => AgentMsg::ResponseCallback(who, CoffeeStoreAgentResponse::ListCoffeeStoresResponse(server_response)),
                    Err(_) => AgentMsg::ResponseCallback(who, CoffeeStoreAgentResponse::ServerError)
                }
            },
        );
        let fetch_task = FetchService::fetch(http_request, http_callback)?;
        self.fetch_tasks.insert(who, fetch_task);
        Ok(())
    }
}

fn create_request<T>(path: &'static str, request: &T) -> anyhow::Result<FetchRequest<Text>> where T: Serialize {
    let text: Text = Ok(serde_json::to_string(request)?);
    Ok(FetchRequest::post(format!("{}{}", SERVER_URL_BASE, path))
        .body(text)
        .unwrap())
}

fn parse_response<'a, T>(response: &'a FetchResponse<Text>) -> Result<T, anyhow::Error> where T: Deserialize<'a> {
    if response.status().is_success() {
        log::info!("SUCCESS!");
        match response.body() {
            Ok(json) => serde_json::from_str(json).map_err(|error| anyhow::Error::new(error)),
            Err(err) => Err(anyhow::Error::new(SimpleError::new(format!("Error parsing the response body: {}", err).to_string())))
        }
    } else {
        Err(anyhow::Error::new(SimpleError::new(format!("Error from server. Code: {}", response.status()).to_string())))
    }
}

impl Agent for CoffeeStoreAgent {
    type Reach = Context<Self>;
    type Message = AgentMsg;
    type Input = CoffeeStoreAgentRequest;
    type Output = CoffeeStoreAgentResponse;

    fn create(link: AgentLink<Self>) -> Self {
        Self {
            link,
            subscribers: HashSet::new(),
            fetch_tasks: HashMap::new()
        }
    }

    fn update(&mut self, msg: Self::Message) {
        match msg {
            AgentMsg::ResponseCallback(who, response) => {
                self.link.respond(who, response);
                self.fetch_tasks.remove(&who);
            }
        }
    }

    fn connected(&mut self, id: HandlerId) {
        self.subscribers.insert(id);
    }

    fn handle_input(&mut self, msg: Self::Input, who: HandlerId) {
        let result = match msg {
            CoffeeStoreAgentRequest::GetCoffeeStore(coffee_store_id) => self.get_coffee_store(who, coffee_store_id),
            CoffeeStoreAgentRequest::ListCoffeeStores(request) => self.list_coffee_stores(who, request.next_token, request.max_items),
            CoffeeStoreAgentRequest::CreateCoffeeStore(manifest) => self.create_coffee_store(who, manifest),
        };

        if result.is_err() {
            log::error!("Failed to send request: {}", result.err().unwrap());
            self.link.respond(who, CoffeeStoreAgentResponse::AgentError)
        }
    }

    fn disconnected(&mut self, id: HandlerId) {
        self.subscribers.remove(&id);
    }
}