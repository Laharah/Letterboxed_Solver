use crate::trie::Trie;
use core::cmp::Reverse;
use indexmap::IndexMap;
use std::collections::BinaryHeap;
use std::collections::VecDeque;

#[derive(Hash, Eq, PartialEq, Debug)]
pub struct Board {
    pub letters: [char; 12],
}

impl Board {
    pub fn show_board(&self) {
        print!("  ");
        for c in self.letters[..4].iter() {
            print!("  {} ", c);
        }
        print!(" ");
        let gap_len = 11;
        let gap = String::from(" ").repeat(gap_len);
        print!("\n  ┌");
        for i in 0usize..3 {
            if i != 2 {
                print!("─┬──")
            } else {
                println!("─┬─┐");
            }
        }
        // println!("  │{}│  ", gap);
        for i in 0usize..3 {
            println!(
                " {}├{}┤{} ",
                self.letters[9..][i],
                gap,
                self.letters[3..6][i]
            );
            if i != 2 {
                println!("  │{}│  ", gap);
            }
        }
        print!("  └");
        for i in 0usize..3 {
            if i != 2 {
                print!("─┴──")
            } else {
                println!("─┴─┘");
            }
        }
        print!("   ");
        for c in self.letters[6..9].iter() {
            print!(" {}  ", c);
        }
        print!("\n\n");
    }
}

impl<T> From<T> for Board
where
    T: IntoIterator<Item = char>,
{
    fn from(input: T) -> Self {
        let mut letters = [' '; 12];
        let mut count: usize = 0;

        for c in input {
            if c == ' ' {
                continue;
            }
            letters[count] = c;
            count += 1;
        }

        if count != 12 {
            panic!("Invalid board");
        }

        Board { letters }
    }
}

#[derive(Hash, Eq, PartialEq, Debug, Clone)]
enum Location {
    Root,
    C(usize),
}

#[derive(Hash, Eq, PartialEq, Debug, Clone)]
struct State<'s> {
    pub path: [&'s str; 5],
    path_len: u8,
    current: Location,
    used_chars: [bool; 12],
    board: &'s Board,
}

impl<'a> PartialOrd for State<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl<'a> Ord for State<'a> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let a = self.used_chars.iter().filter(|&&b| b).count();
        let b = other.used_chars.iter().filter(|&&b| b).count();
        match Reverse(a).cmp(&Reverse(b)) {
            core::cmp::Ordering::Equal => {}
            ord => return ord,
        }

        Reverse(self.path_len).cmp(&Reverse(other.path_len))
    }
}

impl<'a> State<'a> {
    fn evolve(&self, new_word: &'a str) -> State {
        let mut new_state = self.clone();
        for (i, &c) in self.board.letters.iter().enumerate() {
            if new_word.contains(c) {
                new_state.used_chars[i] = true;
            }
        }
        new_state.path[new_state.path_len as usize] = new_word;
        new_state.path_len += 1;
        let last_char = new_word.chars().last().unwrap();
        new_state.current = Location::C(
            self.board
                .letters
                .iter()
                .position(|&c| c == last_char)
                .unwrap(),
        );
        new_state
    }
}

pub fn solve(board: &Board, trie: &Trie) -> Vec<String> {
    let start: State = State {
        path: [""; 5],
        path_len: 0,
        current: Location::Root,
        used_chars: [false; 12],
        board: &board,
    };
    //TODO: use a heap and construct states using a length-first-iterator from trie.
    vec![]
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn construct_board() {
        let _ = Board::from("abcdefghi jkl ".chars());
        let v = vec!['a'; 12];
        let _ = Board::from(v);
    }

    #[test]
    fn board_construction_failure() {
        assert!(std::panic::catch_unwind(|| Board::from("abcdefg".chars())).is_err());
        assert!(std::panic::catch_unwind(|| Board::from("abcdefghi".chars())).is_err());
        assert!(std::panic::catch_unwind(|| Board::from("abcdefghij".chars())).is_err());
        assert!(std::panic::catch_unwind(|| Board::from("abcdefghijk".chars())).is_err());
        assert!(std::panic::catch_unwind(|| Board::from("abcdefghijklm".chars())).is_err());
    }
}
