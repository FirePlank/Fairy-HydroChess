# Fairy-HydroChess
A modified version of the [HydroChess](https://github.com/FirePlank/HydroChess) engine that supports many variants including antichess/suicide, 3check, chess960, racing kings and many more! For a full and up to date list of all the available variants, look in the `src/variants` folder.

The reason for me separating the variants from the base engine is that all the variant checking that has to be done to determine which variant move generation or evalution function to use significantly slow down the engine. I want to keep the standard chess engine as fast as it can, while speed isn't as important for variants.

Fairy-Hydrochess still has the option of playing normal standard chess but its just quite a bit slower than the base engine so if you don't need to use/play any variants I suggest using that.

## Installation

To install Fairy-HydroChess, you will need to have Rust and Cargo installed on your machine. You can find instructions for installing Rust [here](https://www.rust-lang.org/tools/install).

Once you have Rust and Cargo installed, you can install Fairy-HydroChess by cloning this repository and running the following command in the root directory:

`cargo build --release`

This will compile the HydroChess code and create an executable in the `target/release` directory.

## Running Fairy-HydroChess

To run Fairy-HydroChess, you will need to use a chess interface that is compatible with the [Universal Chess Interface (UCI)](https://en.wikipedia.org/wiki/Universal_Chess_Interface) protocol.

Once you have a UCI-compatible chess interface installed, you can start a game by selecting HydroChess as the engine. The specific steps for doing this will depend on the interface you are using.

To play a specific variant you need to make sure to change it with the UCI_Variant command, but most UCI-compatible chess interfaces do this automatically so you just need to choose the variant you want to play within the interface.

## Contributing
I made this project as more of a hobby and more for myself as a challenge, but feel free to suggest any changes or improvements if you so please.
