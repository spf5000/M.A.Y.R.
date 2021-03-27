use yew::{ComponentLink, Component, Callback, ShouldRender, Html, html, Bridge, Bridged, ChangeData};
use yew::events::FocusEvent;
use crate::app_router::{AppRoute, Link};
use crate::agents::coffee_store_agent::{CoffeeStoreAgent, CoffeeStoreAgentRequest, CoffeeStoreAgentResponse};
use rust_server_model::coffee_store::{CoffeeStoreDetails, CoffeeStoreManifest};
use std::default::Default;
use std::collections::HashMap;

const NAME_ID: &str = "name";
const DESCRIPTION_ID: &str = "desc";
const RATING_ID: &str = "rating";
const FIELDS: [(&str, &str, bool); 3] = [
    ("Name:", NAME_ID, true),
    ("Description:", DESCRIPTION_ID, false),
    ("Average Rating:", RATING_ID, false)
];

pub enum Msg {
    FormSubmitted,
    TextEditted(&'static str, ChangeData),
    DataReceived(CoffeeStoreDetails),
    Error(&'static str)
}

pub struct CoffeeStoreForm {
    link: ComponentLink<Self>,
    coffee_summary_agent: Box<dyn Bridge<CoffeeStoreAgent>>,
    coffee_store_manifest_map: HashMap<&'static str, ChangeData>,
    submitted_manifest: Option<CoffeeStoreManifest>,
    coffee_store_details: Option<CoffeeStoreDetails>,
    error: Option<String>
}

impl CoffeeStoreForm {
    fn get_coffee_store_manifest(&self) -> anyhow::Result<CoffeeStoreManifest> {
        let name = self.get_change_data_string(NAME_ID).ok_or(anyhow::Error::msg("Name is required!"))?;
        let description = self.get_change_data_string(DESCRIPTION_ID);
        let avg_rating = self.get_change_data_string(RATING_ID)
            .map(|rating| rating.parse::<f32>().ok())
            .flatten();

        Ok(CoffeeStoreManifest {
            name: name.clone(),
            description: description.cloned(),
            avg_rating
        })
    }

    fn get_change_data_string(&self, id: &'static str) -> Option<&String> {
        let change_data = self.coffee_store_manifest_map.get(id)?;
        if let ChangeData::Value(data) = change_data {
            Some(data)
        } else {
            None
        }

    }
}


impl Component for CoffeeStoreForm {
    type Message = Msg;
    type Properties = ();
    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let callback: Callback<CoffeeStoreAgentResponse> = link.callback(|response: CoffeeStoreAgentResponse| {
            match response {
                CoffeeStoreAgentResponse::CreateCoffeeStoreResponse(details) => Msg::DataReceived(details),
                CoffeeStoreAgentResponse::ServerError => Msg::Error("Server Error"),
                CoffeeStoreAgentResponse::AgentError => Msg::Error("Agent Error"),
                _ => Msg::Error("Unexpected Response received!")
            }
        });
        let agent = CoffeeStoreAgent::bridge(callback);
        Self {
            link,
            coffee_summary_agent: agent,
            coffee_store_manifest_map: HashMap::new(),
            submitted_manifest: None,
            coffee_store_details: None,
            error: None
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::DataReceived(response) => {
                log::info!("Received coffee store details: {:?}", &response);
                self.coffee_store_details = Some(response);
                self.error = None;
                true
            },
            Msg::Error(reason) => {
                self.error = Some(reason.to_string());
                true
            },
            Msg::FormSubmitted => {
                log::info!("Form Submitted. Data: {:?}", &self.coffee_store_manifest_map);
                match self.get_coffee_store_manifest() {
                    Ok(manifest) => {
                        self.coffee_summary_agent.send(CoffeeStoreAgentRequest::CreateCoffeeStore(manifest.clone()));
                        self.submitted_manifest = Some(manifest);

                    },
                    Err(err) => self.error = Some(err.to_string())
                }
                true
            },
            Msg::TextEditted(id, change) => {
                self.coffee_store_manifest_map.insert(id, change);
                true
            }
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
       // If there is an error, we show it
        if self.error.is_some() {
            let error = self.error.as_ref().unwrap();
            render_error(error)
        }
        // If we've gotten a response from the server, show it.
        else if self.coffee_store_details.is_some() {
            let coffee_store_details = self.coffee_store_details.as_ref().unwrap();
            render_success(&coffee_store_details.id)
        }
        // Manifest is submitted, but we don't have details yet. Still waiting on the agent response.
        else if self.submitted_manifest.is_some() {
            render_submitted(self.submitted_manifest.as_ref().unwrap())
        }
        // otherwise, show the user the form to fill out.
        else {
            render_form(&self.link)
        }
    }
}

fn render_error(error: &String) -> Html {
    return html! {
        <div class="components.coffee-summaries-error">
            { format!("Error! {}", error)}
        </div>
    }
}

fn render_submitted(manifest: &CoffeeStoreManifest) -> Html {
    return html! {
        <div class="components.coffee-summaries-subumitted">
            <div> { format!("Submitted Manifest {:?}! Waiting on server response", manifest)} </div>
        </div>
    }
}

fn render_success(id: &String) -> Html {
    return html! {
        <div class="components.coffee-summaries-success">
            <div> { format!("Success! New Coffee Store ID: {}", id)} </div>
            <div><Link route=AppRoute::Home>{"Return Home!"}</Link></div>
        </div>
    }
}

fn render_form(link: &ComponentLink<CoffeeStoreForm>) -> Html {
    let callback: Callback<FocusEvent> = link.callback(|event: FocusEvent| {
        event.prevent_default();
        Msg::FormSubmitted
    });
    return html! {
        <div>
            <div>
                <form onsubmit=callback>
                    { FIELDS.iter().map(|tuple| render_form_field(link, tuple)).collect::<Html>() }
                    <input type="submit", value="Submit"/>
                </form>
            </div>
            <div> <Link route=AppRoute::Home>{"Return Home!"}</Link> </div>
        </div>
    }
}

fn render_form_field(link: &ComponentLink<CoffeeStoreForm>, tuple: &'static (&'static str, &'static str, bool)) -> Html {
    let (field, id, required) = *tuple;
    let callback: Callback<ChangeData> = link.callback(move |change: ChangeData| {
        Msg::TextEditted(id, change)
    });
    return html! {
        <>
            <label for={id}>{field}</label><br/>
            <input type="text" id={id} name={id} onchange=callback required={required} /> <br/>
        </>
    }
}
