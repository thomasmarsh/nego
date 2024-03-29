# Questions about Nego Rules

The game rules of Nego, especially concerning territory capture, are
underspecified.

## Territory Capture

The instructions are limited to the following statements:

> When your NEKOs surround an area, it becomes your territory, and your opponent
can't place pieces there anymore.

> Up to two ends of the board can be used as edges of territory.

> If you capture any spaces with your opponent's pieces inside them, they are
removed from the board and can be played again later. However, Boss NEKO cannot
be removed.

> If you capture spaces that contain or overlap with your opponents territory,
you'll capture all the empty squares, but your opponent will still keep the
territory they've already captured.

This leaves open some questions and we address how those are handled here.
Apparently territory and safety are not dictated by liberties as in go.

### Adjacent Connections

*Are connections between pieces orthogonally adjacencies?*

We surmise so based on limited examples of play. In the following position,
the white kunoji piece at **A7** would be alive.


````
   A B C D E F G H
 1 . . . . . . . . 1
 2 . . . . . . . . 2
 3 . . . . . . . . 3
 4 . . . . . . . . 4
 5 . . . . . . . . 5
 6 X . . . . . . . 6
 7 O X X . . . . . 7
 8 O O X . . . . . 8
   A B C D E F G H
````

If black plays at B6, the white piece would be captured according to our
rules.

On a subjective note, I think this is a little subtle and wonder if a liberties
based approach to capture would be preferable. When defending, it is the
difference between paying attention to piece chains of the opponent vs.
liberties of your pieces.

### Middle Board Territory

*Can spaces be surrounded in the middle of the board?*

We assume yes since it is not specified whether an edge is necessary to capture
territory. Capturing in the center requires more spaces and is less likely, but
could be strategically interesting as the board fills up.

According to our rules, the white mame piece at **E4** is alive, but at risk of
capture.

````
   A B C D E F G H
 1 . . . . . . . . 1
 2 . . . . . . . . 2
 3 . . . X X X . . 3
 4 . . . X O X . . 4
 5 . . . . X X . . 5
 6 . . . . . . . . 6
 7 . . . . . . . . 7
 8 . . . . . . . . 8
   A B C D E F G H
````

This is supported by an example from the
[tutorial video](https://www.youtube.com/watch?v=I8lCrK9Mjtk&t=43s) shows
such a position during scoring.

````
   A B C D E F G H
 1 O O O . . . O O 1
 2 O O X X O O O O 2
 3 X X X X X O O O 3
 4 X . O O O O O O 4
 5 X X O X O O X X 5
 6 X X X X X X O X 6
 7 . X X X X O O O 7
 8 . X X X . O O O 8
   A B C D E F G H
````

The black kunoji group at **G5** would be alive since it is not surrounded by
orthogonally adjacent opponent pieces. (I.e., there is no adjacent connection
between **F5** and **G6**.)


### Territory Extension

*Does a piece played adjacent to secure territory also inherit the safety
of the original capturing group?*

We assume yes since it would be impractical to keep track of the original
extent of a captured territory.

In the following position, the white player has formed a territory in the
upper left with a kunoji. This piece is automatically secure from future
capture since it formed a territory.

````
   A B C D E F G H
 1 . O . . . . . . 1
 2 O O . . . . . . 2
 3 . . . . . . . . 3
 4 . . . . . . . . 4
 5 . . . . . . . . 5
 6 . . . . . . . . 6
 7 . . . . . . . . 7
 8 . . . . . . . . 8
   A B C D E F G H
````

The following nobi move at **C2** would extend its power. Any pieces in the future
could connect to the row of whites on row 2 to guarantee life.

````
   A B C D E F G H
 1 . O . . . . . . 1
 2 O O O O O O . . 2
 3 . . . . . . . . 3
 4 . . . . . . . . 4
 5 . . . . . . . . 5
 6 . . . . . . . . 6
 7 . . . . . . . . 7
 8 . . . . . . . . 8
   A B C D E F G H
````

This seems a reasonable approach and is similar to how go rules work in the
presence of secure territories. It is notable that the boss should *not* extend
its safety to adjacent like colored pieces.

### No Empty Space Capture

*Can a territory be captured if it contains only opponent pieces but no empty
spaces?*

It is clear that you can't capture territory just by playing against a wall.
(I.e., touching a wall does not mark your stone as safe.) We assume, however,
that surrounding a piece is sufficient to capture it without requiring
additional empty spaces in the new territory.

### Filled Territories

An implict result of the above decisions is that filled territories are still safe.
For example, in the following position the kunoji group at **B1** is safe.

````
   A B C D E F G H
 1 . O X . . . . . 1
 2 O O X . . . . . 2
 3 X X X . . . . . 3
 4 . . . . . . . . 4
 5 . . . . . . . . 5
 6 . . . . . . . . 6
 7 . . . . . . . . 7
 8 . . . . . . . . 8
   A B C D E F G H
````

White is permitted to place within their own territory with a mame at **A1**. This
wil not be a suicide.

````
   A B C D E F G H
 1 O O X . . . . . 1
 2 O O X . . . . . 2
 3 X X X . . . . . 3
 4 . . . . . . . . 4
 5 . . . . . . . . 5
 6 . . . . . . . . 6
 7 . . . . . . . . 7
 8 . . . . . . . . 8
   A B C D E F G H
````

This approach is supported by the rule that once a territory is safe, it cannot
be removed from the baord.

## Other Questions

### Should the game be played with komi?

Draws are possible, and perhaps even likely. The rules disallow placing the
boss on the center, presumably to reduce the first move advantage. However,
it's unclear (to me) that the center position is the strongest opening move.
The engine currently assigns komi of 0.5 to prevent draws.

### Can a player pass?

The rules do not stipulate whether a player may pass, so it is assumed they may
not. Passing might be useful in some rare cases and null moves are utilized in
some types of minimax searches.
