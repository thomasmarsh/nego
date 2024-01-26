Notes
=

Implementation notes and reminders to myself, mostly.

Low Level Tweaks and Benchmarking
==

I have done no benchmarking other than looking for flamegraph hotspots. Inlining
hints have been added indiscriminately for now.

This game fits well into `u64` and smaller types so is amenable to branch
free and SIMD techniques - more so than chess. I currently see up to 4 million
moves / sec on my M1 macbook air with a parallel iterative search, so it seems
good enough for now. I think the big wins are not going to be found in the low
level optimizations.

One of the hot spots is `Vec::reserve_for_push`. Having some pre-allocated
buffers lifted to an earlier point or held thread locally might help a lot. Note
that there is a large branching factor of around 1680 moves since each the piece
placements has four potential orientations.

The move is a `u16`, which would result in a maximum move buffer size of around
3360 bytes + 2 bytes (`u16` for length).

There are some opportunities to eliminate branching at the cost of additional
total operations. A good example is testing whether a bit is set before doing
a floodfill. The floodfill already has a bailout condition and will return an
empty bitset. I get the impression that something on the order of a dozen branch
instructions need to be eliminated in the hot path before it is worth it. Some
benchmarks would need to implemented, and the results are likely to be very
architecture specific.

MCTS
==

MCTS is desirable due to the large branching factor of the game and the novelty
of the ruleset. However, it is quite naive without add game specific knowledge.
The minimax-rs library supports more directed policies, so definitely worth
exploring. Currently I can't use MCTS for lengthy explorations because it quickly
runs out of memory.

I should also look into [MCTS-Solver](https://dke.maastrichtuniversity.nl/m.winands/documents/uctloa.pdf)
which might be interesting because it adds value estimation to MCTS. I don't know
if that might reduce the need for a tuned rollout policy.


MuZero
==
I looked into AlphaZero/MuZero. A player might worth exploring for the
[muzero-general](https://github.com/werner-duvaud/muzero-general) sandbox. These
projects are generally hard to run locally without a docker image due to Python/pip
dependency hell, but it could be run on [Google Colab](https://colab.research.google.com).


MTD(f) and minimax-rs
==

[minimax-rs](https://github.com/edre/minimax-rs) supports MTD(f). This prunes a
lot of the search tree and makes play extremely fast. However, it seems to prune
a bit too much, and is not resulting in optimal play. I'm not familiar enough
with tuning and optimizing minimax approaches, so the current configuration is a
bit of guesswork for now.

minimax-rs has some great information in its comments, but it has some bugs and
is underdocumented in places. It is being used in [Nokamute](https://github.com/
edre/nokamute), but apparently nowhere else in GitHub.

The version of minimax-rs we use is forked from an older version which doesn't
include a number of enhancements. Since the last update to 

Game Specific
==

* The game would exhibit strong rotational symmetry except that bosses are
  immobile once place and captures are relatively rare.

* The opening move could be analyzed since the search algorithms have a hard
  time differentiating first moves.

* Evaluation might be stronger if territory capture were scored more highly.

* Transposition tables should greatly reduce the search space. They are
  implemented, but it is unclear if everything is sound.


UI
==

A simple text mode would be nice, with the caveat that visualizing the gaze
of the pieces is hard. For UI, a GTP inspired protocol would allow the UI to
be decoupled. A simple UI is being implemented in Rust since it is hard to
visualize the game from a CLI.
