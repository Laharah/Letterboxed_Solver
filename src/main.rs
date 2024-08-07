use std::io::BufRead;
mod game;
mod trie;
use game::Board;

fn main() {
    ()
    // let b = Board::from("abcdefghijkl".chars());
    // b.show_board();
    // let letters = b.letters.iter().flat_map(|r| r.iter()).collect::<Vec<_>>();
    // let f = std::fs::File::open("/usr/share/dict/american-english").unwrap();
    // let f = std::io::BufReader::new(f);
    // let filtered = f
    //     .lines()
    //     .map(|l| l.unwrap())
    //     .filter(|l| {
    //         if l.len() < 3 || l.ends_with("'s") {
    //             return false;
    //         }
    //         if l.chars().any(|c| !letters.contains(&&c)) {
    //             return false;
    //         }
    //         true
    //     })
    //     .map(|l| l.trim().to_lowercase());
    //
    // // let t = Trie::new(filtered);
    // // println!("{}", t.len());
}
