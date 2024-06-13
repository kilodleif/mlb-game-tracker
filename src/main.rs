use mlb_game_tracker::{metadata, tracker::Tracker};

fn main() {
    metadata::init();

    Tracker::run()
}
