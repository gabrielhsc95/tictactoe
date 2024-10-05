mod board;
mod coordinates;
mod player;
mod random_strategy;
mod terminal;
mod tictactoe;
use clap::{Arg, ArgGroup, Command};
use random_strategy::RandomStrategy;
use std::io;
use terminal::Terminal;
use tictactoe::TicTacToe;

fn main() {
    let matches = Command::new("Tic Tac Toe")
        .version("0.0.1")
        .author("Gabriel Cardoso")
        .about("A simple Tic Tac Toe game to learn Rust.")
        .arg(
            Arg::new("single")
                .short('s')
                .long("single")
                .help("Single player mode")
                .value_parser(["best", "random"])
                .long_help(
                    "Single player mode
The available strategies are:
    best: it won't lose
    random: CPU plays randomly",
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
        .get_matches();

    let terminal = Terminal::new();
    let mut tic_tac_toe = TicTacToe::new(terminal);
    if matches.contains_id("single") {
        let strategy_selection = matches.get_one::<String>("single").unwrap();
        let strategy: RandomStrategy;
        if strategy_selection == "best" {
            panic!("Not implemented yet!");
        } else if strategy_selection == "random" {
            strategy = RandomStrategy::new();
        } else {
            panic!("It should not end up here!");
        }
        tic_tac_toe.play_single(&strategy);
    } else if matches.get_flag("multi") {
        tic_tac_toe.play_multi();
    } else {
        panic!("It should not end up here!");
    }

    println!("Press Enter to exit...");
    match io::stdin().read_line(&mut String::new()) {
        Ok(_) => {}
        Err(error) => println!("Error reading input: {}", error),
    }
}
