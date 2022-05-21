use crate::components::{notification::NotificationProps, notifications::NotificationList};
use std::collections::HashSet;
use yew::Callback;
use yew_agent::{Agent, AgentLink, Context, HandlerId};

#[derive(Clone)]
pub enum NotificationInput {
    Add(NotificationProps),
}

#[derive(Clone)]
pub enum NotificationMessage {
    Add(NotificationProps),
    Del(usize),
}

#[derive(Clone)]
pub enum NotificationResponse {
    Out(NotificationList),
}
#[derive(Clone)]
pub struct NotificationAgent {
    link: AgentLink<Self>,
    subscribers: HashSet<HandlerId>,
    ns: NotificationList,
    next_id: usize,
}

impl NotificationAgent {
    pub fn get_next_id(&mut self) -> usize {
        let id = self.next_id;
        self.next_id += 1;
        id
    }

    fn send(&self, ns: NotificationList) {
        let res = NotificationResponse::Out(ns);
        self.subscribers
            .iter()
            .for_each(|id| self.link.respond(*id, res.clone()))
    }
}

impl Agent for NotificationAgent {
    type Reach = Context<Self>;

    type Message = NotificationMessage;

    type Input = NotificationInput;

    type Output = NotificationResponse;

    fn create(link: AgentLink<Self>) -> Self {
        gloo_console::log!("notification agent create");
        NotificationAgent {
            link,
            subscribers: HashSet::new(),
            ns: NotificationList::new(),
            next_id: 0,
        }
    }

    fn update(&mut self, msg: Self::Message) {
        gloo_console::log!("notification agent update");
        match msg {
            NotificationMessage::Add(mut props) => {
                let next_id = self.next_id;

                props.timeout_callback = {
                    let s = self.clone();
                    Callback::once(move |_| {
                        gloo_console::log!("Notification timeout callback...");
                        s.link.send_message(NotificationMessage::Del(next_id));
                    })
                };

                props.close_callback = {
                    let s = self.clone();
                    Callback::once(move |_| {
                        gloo_console::log!("Notification close callback...");
                        s.link.send_message(NotificationMessage::Del(next_id));
                    })
                };

                let id = self.get_next_id();
                self.ns.insert(0, (id, props));
                self.send(self.ns.clone());
            }
            NotificationMessage::Del(next_id) => {
                for (pos, (id, _)) in self.ns.iter().enumerate() {
                    if *id == next_id {
                        self.ns.remove(pos);
                        gloo_console::log!("Notification removed...");
                        self.send(self.ns.clone());
                        break;
                    }
                }
            }
        }
    }

    fn handle_input(&mut self, msg: Self::Input, _id: HandlerId) {
        gloo_console::log!("notification agent handle input");
        match msg {
            NotificationInput::Add(props) => {
                self.link.send_message(NotificationMessage::Add(props))
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
