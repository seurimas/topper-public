use crate::*;

use super::PersuasionEvent;

pub trait PersuasionTransition {
    fn probable_events(
        &self,
        my_state: &PersuasionState,
        their_status: &PersuasionStatus,
    ) -> Vec<(f32, Vec<PersuasionEvent>)>;
}
pub struct Draw;

impl PersuasionTransition for Draw {
    fn probable_events(
        &self,
        my_state: &PersuasionState,
        _their_status: &PersuasionStatus,
    ) -> Vec<(f32, Vec<PersuasionEvent>)> {
        let deck = if my_state.appeals_in_deck.is_empty() {
            my_state.discarded_appeals.clone()
        } else {
            my_state.appeals_in_deck.clone()
        };

        deck.iter()
            .map(|appeal| {
                let mut events = vec![];
                events.push(PersuasionEvent::Draw(vec![appeal.clone()]));
                (1.0 / deck.len() as f32, events)
            })
            .collect()
    }
}
