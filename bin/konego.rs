use comfy::*;

use nego::{
    agent::Agent,
    core::{
        game::{self, Color::*},
        ray::Rays,
    },
    ui::{draw, piece},
};

#[derive(Clone, Debug)]
pub struct ThreadState(Arc<Mutex<ThreadData>>);

impl ThreadState {
    fn new() -> Self {
        Self(Arc::new(Mutex::new(ThreadData::new())))
    }

    fn get(&self) -> WorkerState {
        self.0.lock().worker_state
    }

    fn set_ready(&self, state: game::State) {
        let mut lock = self.0.lock();
        lock.worker_state = WorkerState::Ready;
        lock.new_state = state;
    }

    fn set_working(&self) {
        self.0.lock().worker_state = WorkerState::Working;
    }

    fn set_done(&self) {
        self.0.lock().worker_state = WorkerState::Done;
    }

    fn set_and_fetch(&self, worker_state: WorkerState) -> game::State {
        let mut lock = self.0.lock();
        lock.worker_state = worker_state;
        lock.new_state.clone()
    }
}

#[derive(Debug)]
pub struct MyGame {
    pub state: game::State,
    pub ui: UIState,
    pub thread_state: ThreadState,
}

#[derive(Debug)]
pub struct UIState {
    show_spinner: bool,
    agent_black: Agent,
    agent_white: Agent,
}

impl UIState {
    fn new() -> Self {
        Self {
            show_spinner: false,
            agent_black: Agent::Iterative(std::time::Duration::from_secs(5)),
            agent_white: Agent::Parallel(std::time::Duration::from_secs(5)),
        }
    }
}

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

impl GameLoop for MyGame {
    fn new(_c: &mut EngineState) -> Self {
        Rays::build_lut();
        Self {
            state: game::State::new(),
            ui: UIState::new(),
            thread_state: ThreadState::new(),
        }
    }

    fn update(&mut self, _c: &mut EngineContext) {
        self.update_state();
        self.draw();
        self.user_input();
    }
}

#[inline]
fn draw_player(state: &game::PlayerState, color: game::Color) {
    state
        .move_list
        .iter()
        .for_each(|m| piece::Parts::new(m.get_piece().piece_type_id()).draw(color, *m));
}

impl MyGame {
    fn draw(&self) {
        draw::board();
        draw_player(&self.state.board.black, Black);
        draw_player(&self.state.board.white, White);
        self.right_panel();
    }

    fn right_panel(&self) {
        egui::SidePanel::right("my_right_panel")
            .default_width(50.)
            .show(egui(), |ui| {
                if self.ui.show_spinner {
                    ui.add(egui::Spinner::new());
                }
            });
    }

    fn current_agent(&self) -> Agent {
        match self.state.current {
            Black => self.ui.agent_black,
            White => self.ui.agent_white,
        }
    }

    fn spawn_worker(&mut self) {
        self.thread_state.set_working();

        let work = self.state.clone();
        let agent = self.current_agent();

        let thread_state = self.thread_state.clone();

        _ = std::thread::spawn(move || {
            if let Some(state) = agent.step(&work) {
                thread_state.set_ready(state);
            } else {
                println!("Score:");
                println!("- black: {}", work.board.black.occupied.popcnt());
                println!("- white: {}", work.board.white.occupied.popcnt());
                thread_state.set_done();
            }
        });
    }

    fn finalize_work(&mut self) {
        self.state = self.thread_state.set_and_fetch(WorkerState::Idle);
        self.state.dump();
    }

    fn update_state(&mut self) {
        if !self.current_agent().is_human() {
            let state = self.thread_state.get();
            self.ui.show_spinner = state == WorkerState::Working;
            match state {
                WorkerState::Idle => self.spawn_worker(),
                WorkerState::Working => (),
                WorkerState::Ready => self.finalize_work(),
                WorkerState::Done => (),
            }
        }
    }

    fn user_input(&self) {
        if !self.current_agent().is_human() {
            return;
        }
        let square_size = 80;
        let Vec2 { x: mx, y: my } = mouse_screen();
        let index = |n: f32| ((n + 40.) / square_size as f32).floor() as i32;
        let (ix, iy) = (index(mx) - 1, index(my) - 1);
        if (0..8).contains(&ix) && (0..8).contains(&iy) {
            let snap = |n: f32| (index(n) * square_size) as f32;
            let snapped_pos = Vec2::new(snap(mx), snap(my));
            draw_rect(
                screen_to_world(snapped_pos),
                screen_to_world(Vec2::new(
                    screen_width() / 2.0 + square_size as f32,
                    screen_height() / 2.0 + square_size as f32,
                )),
                Color::rgba8(0x10, 0xff, 0x11, 0x88),
                1,
            );
        }
    }
}

pub fn _comfy_default_config(config: GameConfig) -> GameConfig {
    config
}

pub async fn run() {
    init_game_config("Konego".to_string(), "v0.0.1", _comfy_default_config);
    let mut engine = EngineState::new();
    let game = MyGame::new(&mut engine);
    run_comfy_main_async(game, engine).await;
}

fn main() {
    color_backtrace::install();

    #[cfg(not(target_arch = "wasm32"))]
    {
        pollster::block_on(run());
    }

    #[cfg(target_arch = "wasm32")]
    {
        wasm_bindgen_futures::spawn_local(run());
    }
}
