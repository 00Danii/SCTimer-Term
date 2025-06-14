use std::time::{Duration, Instant};

#[derive(Debug)]
pub enum TimerState {
    Idle,
    Inspection(Instant),
    ReadyToStart,
    Solving(Instant),
    Solved(Duration),
}

impl Default for TimerState {
    fn default() -> Self {
        TimerState::Idle
    }
}
