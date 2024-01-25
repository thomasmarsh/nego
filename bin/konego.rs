use comfy::*;

use nego::core::game::Color::*;
use nego::core::{game, r#move, ray::Rays};
use nego::ui::{draw, piece};

#[derive(Debug)]
pub struct MyGame {
    pub state: game::State,
    pub ui: UIState,
    pub thread_state: Arc<Mutex<ThreadData>>,
}

#[derive(Debug)]
pub struct UIState {
    show_spinner: bool,
}

impl UIState {
    fn new() -> Self {
        Self {
            show_spinner: false,
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
            thread_state: Arc::new(Mutex::new(ThreadData::new())),
        }
    }

    fn update(&mut self, _c: &mut EngineContext) {
        self.update_state();
        draw::board();
        self.draw();

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

        egui::SidePanel::right("my_right_panel")
            .default_width(50.)
            .show(egui(), |ui| {
                if self.ui.show_spinner {
                    ui.add(egui::Spinner::new());
                }
            });
    }
}

#[inline]
fn draw_one(piece: &r#move::Move, color: game::Color) {
    use nego::core::pieces::PieceTypeId::*;
    let pos = piece.position().get_coord();
    let (x, y) = (pos.0 as u8, pos.1 as u8);
    let dir = piece.orientation();
    use piece::*;
    match piece.get_piece().piece_type_id() {
        Boss => mk_boss(color),
        Mame => mk_mame(color),
        Nobi => mk_nobi(color),
        Koubaku1 => mk_koubaku1(color),
        Koubaku2 => mk_koubaku2(color),
        Koubaku3 => mk_koubaku3(color),
        Kunoji1 => mk_kunoji1(color),
        Kunoji2 => mk_kunoji2(color),
        Kunoji3 => mk_kunoji3(color),
        Kunoji4 => mk_kunoji4(color),
    }
    .translate(x as _, y as _)
    .facing(dir)
    .draw();
}

fn set_work_state(thread_state: &Arc<Mutex<ThreadData>>, state: WorkerState) {
    let mut lock = thread_state.lock();
    lock.worker_state = state;
}

#[inline]
fn draw_player(state: &game::PlayerState, color: game::Color) {
    state.move_list.iter().for_each(|m| draw_one(m, color));
}

impl MyGame {
    fn draw(&self) {
        draw_player(&self.state.board.black, Black);
        draw_player(&self.state.board.white, White);
    }

    fn spawn_worker(&mut self) {
        set_work_state(&self.thread_state, WorkerState::Working);

        let work = self.state.clone();
        let thread_state = self.thread_state.clone();

        _ = std::thread::spawn(move || {
            use nego::agent::AIPlayer::*;

            let timeout = std::time::Duration::from_secs(60);
            let new_state_opt = match work.current {
                Black => Parallel.step(&work, timeout),
                White => Iterative.step(&work, timeout),
            };

            if let Some(new_state) = new_state_opt {
                let mut lock = thread_state.lock();
                lock.worker_state = WorkerState::Ready;
                lock.new_state = new_state;
            } else {
                println!("Score:");
                println!("- black: {}", work.board.black.occupied.popcnt());
                println!("- white: {}", work.board.white.occupied.popcnt());
                set_work_state(&thread_state, WorkerState::Done);
            }
        });
    }

    fn finalize_work(&mut self) {
        {
            let mut lock = self.thread_state.lock();
            lock.worker_state = WorkerState::Idle;
            self.state = lock.new_state.clone();
        }
        self.state.dump();
    }

    fn update_state(&mut self) {
        let state = {
            let lock = self.thread_state.lock();
            lock.worker_state
        };
        self.ui.show_spinner = state == WorkerState::Working;
        match state {
            WorkerState::Idle => self.spawn_worker(),
            WorkerState::Working => (),
            WorkerState::Ready => self.finalize_work(),
            WorkerState::Done => (),
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
