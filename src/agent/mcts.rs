use crate::agent::Nego;
use crate::core::game::State;

use minimax::{Game, MCTSOptions, MonteCarloTreeSearch, RolloutPolicy, Strategy};

use rand::seq::SliceRandom;

struct Policy;

impl RolloutPolicy for Policy {
    type G = Nego;
    fn random_move(
        &self,
        state: &mut <Nego as Game>::S,
        moves: &mut Vec<<Nego as Game>::M>,
        rng: &mut rand::rngs::ThreadRng,
    ) -> <Nego as Game>::M {
        state.get_moves(moves);
        // TODO:
        // - wieghted random choice
        // - increased weight for advantageous move
        // - prefer moves that block potential connections
        // - prefer moves that connect territory
        // - prefer moves that create territory
        *moves.choose(rng).unwrap()
    }
}

pub fn step(state: &State, timeout: std::time::Duration) -> Option<State> {
    let opts = MCTSOptions::default()
        .verbose()
        .with_rollouts_before_expanding(5);

    let mut strategy: MonteCarloTreeSearch<Nego> =
        MonteCarloTreeSearch::new_with_policy(opts, Box::new(Policy));

    strategy.set_timeout(timeout);

    let mut new_state = state.clone();
    strategy
        .choose_move(&new_state)
        .and_then(|m| Nego::apply(&mut new_state, m))
}
