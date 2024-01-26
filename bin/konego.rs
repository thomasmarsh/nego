use comfy::{init_game_config, pollster, run_comfy_main_async, EngineState, GameConfig, GameLoop};

use nego::ui::app::Konego;

pub fn _comfy_default_config(config: GameConfig) -> GameConfig {
    config
}

pub async fn run() {
    init_game_config("Konego".to_string(), "v0.0.1", _comfy_default_config);
    let mut engine = EngineState::new();
    let game = Konego::new(&mut engine);
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
