mod game;
mod non_nan;
mod trie;

use game::{solve, Board};
use std::env;
use trie::Trie;

fn main() {
    // Get board letters from command-line arguments
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <board_letters>", args[0]);
        std::process::exit(1);
    }
    let board_letters = &args[1];

    let board = Board::from(board_letters.chars());
    let file = include_str!("2of12.txt");

    let words = file.lines().filter_map(|l| {
        if l.len() < 3 || l.ends_with("'s") {
            None
        } else {
            Some(l.trim().to_lowercase())
        }
    });

    let trie = Trie::new_with_board(words, &board);
    println!(
        "There are {} words that can be made with this board.",
        trie.len()
    );

    let answer = solve(&board, &trie);
    println!("{:?}", answer);
}
