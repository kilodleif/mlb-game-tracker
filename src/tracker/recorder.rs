use std::sync::mpsc::{Receiver, Sender};
use std::sync::Mutex;
use crate::tracker::{InputSignal, OutputSignal};

pub(super) struct Recorder {
    receiver: Mutex<Receiver<OutputSignal>>,
    sender: Sender<InputSignal>
}

impl Recorder {

    pub(super) fn init(receiver: Mutex<Receiver<OutputSignal>>, sender: Sender<InputSignal>) -> Self {

        Recorder { receiver, sender }
    }

    pub(super) fn run(&self) {
        match self.receiver.lock().unwrap().recv().unwrap() {
            OutputSignal::PrintScoreBoard(schedule) => {
                schedule.print_current_date_schedule()
            }
            OutputSignal::PrintOnePlay(play) => {

            }
            OutputSignal::End => {
                println!("Bye!")
            }
        }
    }

}