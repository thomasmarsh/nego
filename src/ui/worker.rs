use crate::{agent::Agent, core::game};

use std::sync::{Arc, Mutex};

#[repr(u8)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum WorkerState {
    Idle,
    Working,
    Ready,
    Done,
}

#[derive(Debug)]
struct ThreadData {
    new_state: game::State,
    worker_state: WorkerState,
}

impl ThreadData {
    fn new() -> Self {
        Self {
            new_state: game::State::new(),
            worker_state: WorkerState::Idle,
        }
    }
}

#[derive(Clone, Debug)]
pub struct Worker(Arc<Mutex<ThreadData>>);

impl Default for Worker {
    fn default() -> Self {
        Self::new()
    }
}

impl Worker {
    pub fn new() -> Self {
        Self(Arc::new(Mutex::new(ThreadData::new())))
    }

    pub fn get_state(&self) -> WorkerState {
        self.0.lock().unwrap().worker_state
    }

    pub fn set_idle_and_fetch(&self) -> game::State {
        let mut lock = self.0.lock().unwrap();
        lock.worker_state = WorkerState::Idle;
        lock.new_state.clone()
    }

    pub fn spawn(&mut self, game_state: game::State, agent: Agent) {
        self.set_working();
        let worker = self.clone();

        _ = std::thread::spawn(move || {
            if let Some(state) = agent.step(&game_state) {
                worker.set_ready(state);
            } else {
                println!("Score:");
                println!("- black: {}", game_state.board.black.occupied.popcnt());
                println!("- white: {}", game_state.board.white.occupied.popcnt());
                worker.set_done();
            }
        });
    }

    fn set_ready(&self, state: game::State) {
        let mut lock = self.0.lock().unwrap();
        lock.worker_state = WorkerState::Ready;
        lock.new_state = state;
    }

    fn set_working(&self) {
        self.0.lock().unwrap().worker_state = WorkerState::Working;
    }

    fn set_done(&self) {
        self.0.lock().unwrap().worker_state = WorkerState::Done;
    }
}
