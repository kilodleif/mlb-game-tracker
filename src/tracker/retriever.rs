use std::sync::mpsc::{Receiver, Sender};
use std::sync::{Mutex};

use reqwest::blocking::Client;

use crate::entity::Schedule;

use crate::constant::API_BASE_URL;
use crate::tracker::{InputSignal, OutputSignal};

const CUR_SCHEDULE_API_URL: &str = "/api/v1/schedule?sportId=1";

pub(super) struct Retriever {
    sender: Sender<OutputSignal>,
    receiver: Mutex<Receiver<InputSignal>>,
    client: Client
}

impl Retriever {
    pub(super) fn init(sender: Sender<OutputSignal>, receiver: Mutex<Receiver<InputSignal>>) -> Self {
        let client = Client::new();

        Retriever{ sender, receiver, client }
    }

    pub(super) fn run(&self) {
        let schedule = self.retrieve_schedule_from_server();
        self.sender.send(OutputSignal::PrintScoreBoard(schedule)).unwrap();
        match self.receiver.lock().unwrap().recv().unwrap() {

        }
    }

    fn retrieve_schedule_from_server(&self) -> Schedule {
        self.client.get(format!("{API_BASE_URL}{CUR_SCHEDULE_API_URL}")).send().unwrap().json::<Schedule>().unwrap()
    }


}