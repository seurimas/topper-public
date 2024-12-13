use topper_core::timeline::{BaseAgentState, NonAgentState, Timeline};

pub struct TimelineModule<O, P, A, N> {
    pub timeline: Timeline<O, P, A, N>,
}

impl<O, P, A: BaseAgentState + Clone, N: NonAgentState + Clone> TimelineModule<O, P, A, N> {
    pub fn new() -> Self {
        TimelineModule {
            timeline: Timeline::<O, P, A, N>::new(),
        }
    }
}
