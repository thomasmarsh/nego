use comfy::*;

use crate::{
    agent::Agent,
    core::{
        game::{self, Color::*},
        ray::Rays,
    },
    ui::{
        draw, piece,
        worker::{ThreadState, WorkerState},
    },
};

#[derive(Debug, Default)]
pub struct Konego {
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

impl Default for UIState {
    fn default() -> Self {
        Self::new()
    }
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

impl GameLoop for Konego {
    fn new(_c: &mut EngineState) -> Self {
        Rays::build_lut();
        Default::default()
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

impl Konego {
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
