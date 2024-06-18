use std::sync::{mpsc, Mutex};
use std::thread;

use crate::entity::Schedule;
use crate::entity::Play;
use crate::tracker::recorder::Recorder;
use crate::tracker::retriever::Retriever;

///
/// to retrieve the stats info from mlb.com
///
mod retriever;

///
/// to record the info to the standard output
///
mod recorder;


enum OutputSignal {

    PrintScoreBoard(Schedule),

    PrintOnePlay(Play),

    //...

    End,

}

enum InputSignal {
    TrackGame(String),
}

pub struct Tracker;

impl Tracker {

    pub fn run() {
        let (output_sender, output_receiver) = mpsc::channel::<OutputSignal>();
        let (input_sender, input_receiver) = mpsc::channel::<InputSignal>();

        let output_receiver = Mutex::new(output_receiver);
        let input_receiver = Mutex::new(input_receiver);

        let retriever = Retriever::init(output_sender, input_receiver);
        let recorder = Recorder::init(output_receiver, input_sender);


        let retriever_handle = thread::spawn(move||retriever.run());
        let recorder_handle = thread::spawn(move||recorder.run());

        retriever_handle.join().unwrap();
        recorder_handle.join().unwrap();
    }

}