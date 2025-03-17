use serde_json::from_reader;
use std::env;
use std::fs::File;
use std::io::BufReader;
use topper_persuasion::{
    simulation::{PersuasionEvent, TimestampedEvent},
    PersuasionState, PersuasionStatus,
};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <file_path>", args[0]);
        std::process::exit(1);
    }

    let file_path = &args[1];
    let file = File::open(file_path).expect("Unable to open file");
    let reader = BufReader::new(file);

    let events: Vec<TimestampedEvent> = from_reader(reader).expect("Unable to parse JSON");
    if let Some(scrutinised) = events
        .iter()
        .find(|event| matches!(event.event, PersuasionEvent::Scrutinised { .. }))
    {
        let TimestampedEvent { time, event } = scrutinised;
        let PersuasionEvent::Scrutinised(_, personality, resolve, max_resolve) = event else {
            panic!("No scrutinised event found");
        };

        let mut myself = PersuasionState::default();
        let mut their_status = PersuasionStatus::Scrutinised {
            resolve: *resolve,
            max_resolve: *max_resolve,
            personality: *personality,
            weakened: vec![],
            unique: false,
        };
        for event in events {
            event.event.apply(&mut myself, &mut their_status);
        }
        println!("Final stats: {:?}", myself);
        println!("Final status: {:?}", their_status);
    } else {
        println!("Found {} events, but no scrutinised event", events.len());
    }
}
