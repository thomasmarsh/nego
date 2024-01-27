use comfy::{egui, EngineContext, EngineState, GameLoop};

use crate::{
    agent::Agent,
    core::{
        game::{self, Color::*},
        r#move::Move,
        ray::Rays,
    },
    ui::{
        draw, piece,
        worker::{Worker, WorkerState},
    },
};

// TODO: make this a tree. For now a history stack.
#[derive(Debug)]
pub struct History {
    current: game::State,
    moves: Vec<Move>,
    states: Vec<game::State>,
}

impl History {
    fn new() -> History {
        History {
            current: game::State::new(),
            moves: Vec::new(),
            states: Vec::new(),
        }
    }

    fn last(&self) -> &game::State {
        &self.current
    }

    fn push(&mut self, value: (game::State, Option<Move>)) {
        if let Some(m) = value.1 {
            self.moves.push(m);
        }
        self.states.push(self.current.clone());
        self.current = value.0;
    }
}

impl Default for History {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Default)]
pub struct Konego {
    pub history: History,
    pub ui: UIState,
    pub worker: Worker,
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
        draw_player(&self.history.last().board.black, Black);
        draw_player(&self.history.last().board.white, White);
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
        match self.history.last().current {
            Black => self.ui.agent_black,
            White => self.ui.agent_white,
        }
    }

    fn finalize_work(&mut self) {
        self.history.push(self.worker.set_idle_and_fetch());
        self.history.last().dump();
        print!("history");
        self.history
            .moves
            .iter()
            .for_each(|m| print!(" {}", m.notation()));
        println!();
    }

    fn update_state(&mut self) {
        if !self.current_agent().is_human() {
            let state = self.worker.get_state();
            self.ui.show_spinner = state == WorkerState::Working;
            match state {
                WorkerState::Idle => self.worker.spawn(self.history.last(), self.current_agent()),
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
        let comfy::Vec2 { x: mx, y: my } = comfy::mouse_screen();
        let index = |n: f32| ((n + 40.) / square_size as f32).floor() as i32;
        let (ix, iy) = (index(mx) - 1, index(my) - 1);
        if (0..8).contains(&ix) && (0..8).contains(&iy) {
            let snap = |n: f32| (index(n) * square_size) as f32;
            let snapped_pos = comfy::Vec2::new(snap(mx), snap(my));
            comfy::draw_rect(
                comfy::screen_to_world(snapped_pos),
                comfy::screen_to_world(comfy::Vec2::new(
                    comfy::screen_width() / 2.0 + square_size as f32,
                    comfy::screen_height() / 2.0 + square_size as f32,
                )),
                comfy::Color::rgba8(0x10, 0xff, 0x11, 0x88),
                1,
            );
        }
    }
}
