use std::io::BufRead;
mod game;
mod trie;
use game::{solve, Board};
use std::fs::File;
use std::io::BufReader;
use trie::Trie;

fn main() {
    // TODO: Get board letters from stdin
    let board = Board::from("omturifahgpl".chars());
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

    let answer = solve(&board, &t);
    println!("{:?}", answer);
}
