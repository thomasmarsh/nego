use crate::agent::Nego;
use crate::core::{game::State, r#move::Move};

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
        assert!(moves.is_empty());
        state.get_moves(moves);
        // TODO:
        // - wieghted random choice
        // - increased weight for advantageous move
        // - prefer moves that block potential connections
        // - prefer moves that connect territory
        // - prefer moves that create territory
        // - add randomness for exploration?
        *moves.choose(rng).unwrap()
    }
}

pub fn step(state: &State, timeout: std::time::Duration) -> Option<Move> {
    let opts = MCTSOptions::default()
        .verbose()
        .with_max_rollout_depth(1000)
        .with_rollouts_before_expanding(5);

    let mut strategy: MonteCarloTreeSearch<Nego> =
        MonteCarloTreeSearch::new_with_policy(opts, Box::new(Policy));

    strategy.set_timeout(timeout);
    strategy.choose_move(state)
}
