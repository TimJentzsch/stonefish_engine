# Stonefish Engine

Stonefish is an amateur chess engine written in Rust.

## Usage

### Lichess

The engine is available on Lichess as [StonefishEngine](https://lichess.org/@/StonefishEngine).

Feel free to challenge it to games when it is online!

### Chess GUI

The engine implements the UCI protocol and should work with most GUIs.

To use it, obtain a binary of the engine (see below) and configure it in a program of your choice!

### Compile From Source

Here's how to compile the engine locally:

1. Install [Rust](https://www.rust-lang.org/learn/get-started).
2. Clone the project:
     
    ```sh
    # Using HTTPS
    git clone https://github.com/TimJentzsch/stonefish_engine.git
    # Using SSH
    git clone git@github.com:TimJentzsch/stonefish_engine.git
    # Using GitHub CLI
    gh repo clone TimJentzsch/stonefish_engine
    ```
3. Move into the folder:

    ```sh
    cd stonefish_engine
    ```
4. Build the project:

    ```sh
    cargo build --release
    ```

You will then find the compiled program in `stonefish_engine/target/release`.

## Features

- Supports the [Universal Chess Interface](https://backscattering.de/chess/uci/) (UCI).
- [Minimax](https://en.wikipedia.org/wiki/Minimax) search with [alpha–beta pruning](https://en.wikipedia.org/wiki/Alpha%E2%80%93beta_pruning) and [iterative deepening](https://en.wikipedia.org/wiki/Iterative_deepening_depth-first_search).
- Heuristic evaluation of material value.

## Resources

- [Pleco](https://github.com/sfleischman105/Pleco) by [@sfleischman105](https://github.com/sfleischman105https://github.com/sfleischman105) for board representation and move generation.
- [UCI Protocol](https://backscattering.de/chess/uci/) for engine–GUI communication.

## License

This project is available under the [GPL-3.0](LICENSE) license.
