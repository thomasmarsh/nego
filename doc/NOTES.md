Notes
=

Implementation notes and reminders to myself, mostly.

Low Level Tweaks and Benchmarking
==

I have done no benchmarking other than looking for flamegraph hotspots. Inlining
hints have been added indiscriminately for now.

This game fits well into `u64` and smaller types so is amenable to branch free and
SIMD techniques - more so than chess. I currently see up to 4 million moves /
sec on my M1 macbook air with a parallel iterate search, so it seems good enough
for now.

One of the hot spots is `Vec::reserve_for_push`. Having some pre-allocated
buffers lifted to an earlier point or held thread locally might help a lot.
Note that there is a large branching factor of around 1680 moves since each the
piece placements has four potential orientations. The move can be packed into a
`u16`, so that would result in a maximum move buffer size of around 3360 bytes + 2
bytes (`u16` for length).

The `State` type contains a `PlayerState` holding a `Vec<Move>`. This could be
optimized a bit. It is set to reserve room for 12 entries. This could be trimmed
to 24 bytes with packed `Move` types, and more if you accept irregular width
entries.

MCTS
==

MCTS is desirable due to the large branching factor of the game and the novelty
of the ruleset. However, it is quite naive without add game specific knowledge.
The minimax-rs library supports more directed policies, so definitely worth
exploring. Currently I can't use MCTS for length explorations because it quickly
runs out of memory.

Game Specific
==

* The game would exhibit strong rotational symmetry except that bosses are
  immobile once place and captures are relatively rare.

* The opening move could be analyzed since the search algorithms have a hard
  time differentiating first moves.

* Evaluation might be stronger if territory capture were scored more highly.


UI
==

A simple text mode would be nice, with the caveat that visualizing the gaze
of the pieces is hard. For UI, a GTP inspired protocol would allow the UI to
be decoupled.
