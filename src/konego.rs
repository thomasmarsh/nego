mod ai;
mod core;
mod ui;

use core::ray::Rays;

use comfy::*;

use crate::core::{game, r#move};
use crate::ui::{draw, piece};

// As in other state-based example we define a global state object
// for our game.
#[derive(Debug)]
pub struct MyGame {
    pub pieces: Vec<piece::Piece>,
    pub state: game::State,
}

// Everything interesting happens here.
impl GameLoop for MyGame {
    fn new(_c: &mut EngineState) -> Self {
        Rays::build_lut();
        // let piece_id = core::pieces::PieceId::Kunoji4;
        // let mut state = game::State::new();
        // state.place(r#move::Move::new(
        // piece_id,
        // core::move_tab::LUTEntry(piece_id.piece_type_id().def().lut_offset),
        // ));

        Self {
            pieces: vec![],
            state: game::State::new(),
        }
    }

    fn update(&mut self, _c: &mut EngineContext) {
        let new_state_opt = match self.state.current {
            game::Color::Black => crate::ai::step_random(&self.state),
            game::Color::White => crate::ai::step_iterative(&self.state),
        };
        if let Some(new_state) = new_state_opt {
            self.state = new_state;
        }
        self.state.dump();
        self.draw();
    }
}

impl MyGame {
    fn draw(&self) {
        draw::board();

        #[inline]
        fn draw_one(piece: &r#move::Move, color: game::Color) {
            use core::pieces::PieceTypeId::*;
            let pos = piece.position().get_coord();
            let (x, y) = (pos.0 as u8, pos.1 as u8);
            let dir = piece.orientation();
            let drawable = match piece.get_piece().piece_type_id() {
                Boss => piece::mk_boss(color),
                Mame => piece::mk_mame(color),
                Nobi => piece::mk_nobi(color),
                Koubaku1 => piece::mk_koubaku1(color),
                Koubaku2 => piece::mk_koubaku2(color),
                Koubaku3 => piece::mk_koubaku3(color),
                Kunoji1 => piece::mk_kunoji1(color),
                Kunoji2 => piece::mk_kunoji2(color),
                Kunoji3 => piece::mk_kunoji3(color),
                Kunoji4 => piece::mk_kunoji4(color),
            }
            .translate(x as _, y as _)
            .facing(dir);

            println!("input: {:}", piece);
            println!("output: {:?}", drawable);
            drawable.draw();
        }
        self.state
            .board
            .black
            .move_list
            .iter()
            .for_each(|m| draw_one(m, game::Color::Black));
        self.state
            .board
            .white
            .move_list
            .iter()
            .for_each(|m| draw_one(m, game::Color::White));
    }
}

// -------------------------------------------------------------------
// The following is the `define_main!()` macro used in other examples,
// expanded for extra clarity.
//
// This isn't likely what most users will want, but it shows that
// comfy can be used without any macros or magic.
//
// We currently don't provide a way to return control over the main game loop
// to the user because of how winit's event loop works. Internally when
// `run_comfy_main_async(...)` is called it ends up calling `event_loop.run(...)`
// on winit, which ends up blocking forever.
// -------------------------------------------------------------------

pub fn _comfy_default_config(config: GameConfig) -> GameConfig {
    config
}

pub async fn run() {
    // comfy includes a `define_versions!()` macro that creates a `version_str()`
    // function that returns a version from cargo & git.
    init_game_config("Konego".to_string(), "v0.0.1", _comfy_default_config);

    let mut engine = EngineState::new();
    // We can do whatever initialization we want in this case.
    let game = MyGame::new(&mut engine);

    run_comfy_main_async(game, engine).await;
}

fn main() {
    #[cfg(feature = "color-backtrace")]
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
