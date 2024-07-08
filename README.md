# Sudoku solver

This program is a sudoku solver written in Rust. It is a TUI application written using Ratatui and Crossterm. The solving algorithm itself is of course UI-agnostic.

## The algorithm

This solver uses a combination of a human-like solving algorithm and a straightforward recursive bruteforce search with backtracking. It treats each cell of the game board as a filled cells are set to colist of numbers that can be put in the cell (`SolvingBoard` and `SolvingCell` in the code). By default filled cells contain only one possible number (their value) and empty cell contain all possible values (candidates). Then, human-like algorithm eliminates the candidates usinga few simple techniques until it cannot do it anymore. If there's only one candidate left in a cell it is considered solved (definitive). This human-like algorithm is enough to solve most sudoku puzzles, but not the hard ones.

That's when the bruteforce algorithm begins it's work. This algorithm is very common and is described in many sources (e. g. [Wikipedia](https://en.wikipedia.org/wiki/Sudoku_solving_algorithms#Backtracking), [Numberphile](https://www.youtube.com/watch?v=G_UYXzGuqvM)). However, this algorithm has a flaw: it's slow and has a few pathological cases where it's especially slow. To fix this, in this program it is reusing information from the previous algorithm, i. e. candidates for each cell, to speed up the bruteforce. This helps to significantly improve its performance. For the most difficult pathological case I could find ([Section 4.2 of this paper](https://www.dcc.fc.up.pt/%7Eacm/sudoku.pdf)), which takes minutes to do with only bruteforce, my combined algorithm only takes hundreds of milliseconds.
