mod board;
mod non_nan;
mod solver;
mod trie;

use board::Board;
use clap::Parser;
use solver::solve;
use trie::Trie;

#[derive(Parser)]
struct Cli {
    ///The 12 letters on the board, clockwise.
    board_letters: String,
    #[clap(short, long, value_hint = clap::ValueHint::FilePath)]
    /// Use custom dictionary file.
    dictionary: Option<String>,
    /// Show the words that can be made with this board.
    #[clap(long)]
    show_words: bool,
}

fn main() {
    // Get board letters from command-line arguments
    let cli = Cli::parse();
    let board_letters = cli.board_letters;

    let board = Board::from(board_letters.chars());
    board.show();

    let words = match &cli.dictionary {
        Some(path) => std::fs::read_to_string(path).expect("Could not read file"),
        None => include_str!("2of12.txt").to_string(),
    };

    let words = words.lines().filter_map(|l| {
        if l.len() < 3 || l.ends_with("'s") {
            None
        } else {
            Some(l.trim().to_lowercase())
        }
    });

    let trie = Trie::new_with_board(words, &board);
    if cli.show_words {
        for word in trie.iter() {
            println!("{}", word);
        }
    }
    println!(
        "There are {} words that can be made with this board.",
        trie.len()
    );

    let answer = solve(&board, &trie);
    let mut code = 0;
    if let Some(solution) = answer {
        let solution = solution
            .into_iter()
            .map(|s| s.to_uppercase())
            .collect::<Vec<String>>()
            .join(", ");
        println!("Found solution: {}", solution);
    } else {
        println!("No solution found");
        code = 1;
    }
    std::process::exit(code);
}
