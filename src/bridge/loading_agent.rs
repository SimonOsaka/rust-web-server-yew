use gloo::console;
use std::collections::HashSet;
use yew_agent::{Agent, AgentLink, Context, HandlerId};

use crate::components::loading::LoadingProps;

#[derive(Clone)]
pub enum LoadingInput {
    Input(LoadingProps),
}

#[derive(Clone)]
pub enum LoadingMessage {
    Open,
    Close,
}

#[derive(Clone)]
pub enum LoadingResponse {
    Out(bool),
}
#[derive(Clone)]
pub struct LoadingAgent {
    link: AgentLink<Self>,
    subscribers: HashSet<HandlerId>,
}

impl LoadingAgent {
    pub fn send(&self, loading: bool) {
        let res = LoadingResponse::Out(loading);
        self.subscribers
            .iter()
            .for_each(|id| self.link.respond(*id, res.clone()))
    }
}

impl Agent for LoadingAgent {
    type Reach = Context<Self>;

    type Message = LoadingMessage;

    type Input = LoadingInput;

    type Output = LoadingResponse;

    fn create(link: AgentLink<Self>) -> Self {
        console::log!("loading agent create");
        LoadingAgent {
            link,
            subscribers: HashSet::new(),
        }
    }

    fn update(&mut self, msg: Self::Message) {
        console::log!("Loading agent update");
        match msg {
            LoadingMessage::Open => {
                self.send(true);
            }
            LoadingMessage::Close => {
                self.send(false);
            }
        }
    }

    fn handle_input(&mut self, msg: Self::Input, _id: HandlerId) {
        console::log!("Loading agent handle input");
        match msg {
            LoadingInput::Input(props) => {
                if props.loading {
                    self.link.send_message(LoadingMessage::Open)
                } else {
                    self.link.send_message(LoadingMessage::Close)
                }
            }
        }
    }

    fn connected(&mut self, id: HandlerId) {
        self.subscribers.insert(id);
    }

    fn disconnected(&mut self, id: HandlerId) {
        self.subscribers.remove(&id);
    }
}
