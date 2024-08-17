mod game;
mod non_nan;
mod trie;

use game::{solve, Board};
use std::env;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
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
    let f = File::open("/home/jaredanderson/Downloads/2of12.txt").unwrap();
    let f = BufReader::new(f);

    let words = f.lines().filter_map(|l| {
        let w = match l {
            Ok(w) => w,
            _ => return None,
        };
        if w.len() < 3 || w.ends_with("'s") {
            None
        } else {
            Some(w.trim().to_lowercase())
        }
    });
    let t = Trie::new_with_board(words, &board);
    println!(
        "There are {} words that can be made with this board.",
        t.len()
    );

    let answer = solve(&board, &t);
    println!("{:?}", answer);
}
