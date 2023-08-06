# Mazes for Programmers in Rust

[![Mazes for Programmers](https://github.com/artemis-beta/mazesfp-rs/actions/workflows/rust_test.yml/badge.svg)](https://github.com/artemis-beta/mazesfp-rs/actions/workflows/rust_test.yml)

This repository contains code following the outline algorithms in _Mazes for Programmers_ by Jamis Buck, which can be found [here](http://www.mazesforprogrammers.com/).

_mazegen_ is a library containing maze construction algorithms which can be applied to a grid struct.

## Algorithms

For all these approaches a grid of squares is constructed. In this context the term 'wall' refers to the boundary between two cells within the grid.

### Binary Tree

In the binary tree algorithm each cell of the grid is visited and a coin is tossed, the result determines if either the northern or eastern wall is erased (where possible). This is the simplest algorithm, although not the best in terms of results.

Run the example with:
```sh
cargo run --example binary_tree
```

### Sidewinder

In the sidewinder algorithm a coin is again tossed, however if the result is heads, the eastern wall is erased and the cell is added to a list of visited cells. If the result is tails, a random cell from those visited is chosen and the southern boundary is erased, the visited cell list is then cleared. If the cell is an eastern boundary cell it is treated as tails.

Run the example with:
```sh
cargo run --example sidewinder
```
