//! This is nothing more than a Tic Tac Toe game.
//!
//! ## Why?
//!
//! I thought it could be a good toy project to learn the language, because I can go
//! as simple as having something that kinda runs in multiplayer mode, to implementing
//! an AI CPU player.
//!
//! As a result, there will be a lot of things that will be over done, over engineered.
//! Stuff that borderline unnecessary, like this documentation, but it my way to explore
//! the features of the Rust language.
//!
//! ## How to
//!
//! Windows multiplayer
//! ```console
//! tictactoe.exe -m
//! ```
//!
//! Windows single player
//! ```console
//! tictactoe.exe -s random
//! ```
//! The argument is the strategy for the CPU.
//!
//! ## TODO
#![doc = include_str!("../todo.md")]
mod bcoodinate;
mod bgame;
mod board;
mod bstrategy;
mod bui;
mod coordinate;
mod game;
mod player;
mod strategy;
mod ui;
use clap::{Arg, ArgGroup, Command};
use std::io;
mod butils;
mod error;

pub use crate::error::{Error, Result};

/// Entrypoint for the Tic Tac Toe game
fn main() -> Result<()> {
    let matches = Command::new("Tic Tac Toe")
        .version("0.0.1")
        .author("Gabriel Cardoso")
        .about("A simple Tic Tac Toe game to learn Rust.")
        .arg(
            Arg::new("single")
                .short('s')
                .long("single")
                .help("Single player mode")
                .value_parser(["best", "random", "medium"])
                .long_help(
                    "Single player mode
The available strategies are:
    best: it won't lose
    random: CPU plays randomly
    medium: random if cannot win",
                ),
        )
        .arg(
            Arg::new("multi")
                .short('m')
                .long("multi")
                .help("Multiplayer mode")
                .action(clap::ArgAction::SetTrue),
        )
        .group(
            ArgGroup::new("mode")
                .args(["single", "multi"])
                .required(true),
        )
        .arg(
            Arg::new("first")
                .short('f')
                .long("first")
                .help("Go first in single player mode")
                .action(clap::ArgAction::SetTrue)
                .requires("single")
                .conflicts_with("multi"),
        )
        .get_matches();

    let ui = ui::tui::TerminalUserInterface::new();
    let mut tic_tac_toe = game::Game::new(ui);
    if matches.contains_id("single") {
        let strategy_selection = matches
            .get_one::<String>("single")
            .expect("The argument parser should have panic before!");
        let strategy: Box<dyn strategy::Strategy>;
        if strategy_selection == "best" {
            strategy = Box::new(strategy::best::BestStrategy::new());
        } else if strategy_selection == "random" {
            strategy = Box::new(strategy::random::RandomStrategy::new());
        } else if strategy_selection == "medium" {
            strategy = Box::new(strategy::medium::MediumStrategy::new());
        } else {
            unreachable!(
                "All arguments for strategy selection should have been exhaustively listed!"
            );
        }
        if matches.get_flag("first") {
            tic_tac_toe.play_single_first(&*strategy);
        } else {
            tic_tac_toe.play_single_second(&*strategy);
        }
    } else if matches.get_flag("multi") {
        let bui = bui::btui::BinaryTerminalUserInterface {};
        let mut tic_tac_toe = bgame::BinaryGame::new(bui);
        tic_tac_toe.play_multi();
    } else {
        unreachable!("There is no option besides single and multiplayer");
    }

    println!("Press Enter to exit...");
    match io::stdin().read_line(&mut String::new()) {
        Ok(_) => {}
        Err(error) => println!("Error reading input: {error}"),
    }

    Ok(())
}
