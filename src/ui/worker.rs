use crate::core::game;

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
pub struct ThreadData {
    pub new_state: game::State,
    pub worker_state: WorkerState,
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
pub struct ThreadState(Arc<Mutex<ThreadData>>);

impl Default for ThreadState {
    fn default() -> Self {
        Self::new()
    }
}

impl ThreadState {
    pub fn new() -> Self {
        Self(Arc::new(Mutex::new(ThreadData::new())))
    }

    pub fn get(&self) -> WorkerState {
        self.0.lock().unwrap().worker_state
    }

    pub fn set_ready(&self, state: game::State) {
        let mut lock = self.0.lock().unwrap();
        lock.worker_state = WorkerState::Ready;
        lock.new_state = state;
    }

    pub fn set_working(&self) {
        self.0.lock().unwrap().worker_state = WorkerState::Working;
    }

    pub fn set_done(&self) {
        self.0.lock().unwrap().worker_state = WorkerState::Done;
    }

    pub fn set_and_fetch(&self, worker_state: WorkerState) -> game::State {
        let mut lock = self.0.lock().unwrap();
        lock.worker_state = worker_state;
        lock.new_state.clone()
    }
}
