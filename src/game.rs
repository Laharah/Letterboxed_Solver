use crate::non_nan::OrderedF32;
use crate::trie::Trie;
use indexmap::IndexMap;
use std::collections::BinaryHeap;

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
    pub fn get_idx(&self, c: char) -> usize {
        self.letters
            .iter()
            .enumerate()
            .find(|x| *x.1 == c)
            .unwrap()
            .0
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
    Idx(usize),
}

#[derive(Hash, Debug, Eq, PartialEq)]
struct State<'b> {
    board: &'b Board,
    word: String,
    path_len: usize,
    used_chars: [bool; 12],
    location: Location,
}

impl<'b> State<'b> {
    fn get_child_states(&self, trie: &Trie) -> Vec<State<'b>> {
        if self.path_len >= 5 {
            return vec![];
        }
        let iter = match self.word.chars().last() {
            Some(c) => trie.iter_from(&c.to_string()),
            None => trie.iter(),
        };
        let start_char = self.word.chars().last();
        let illegle = match start_char {
            None => &self.board.letters[0..0],
            Some(c) => {
                let start_idx = self.board.get_idx(c) / 3;
                &self.board.letters[start_idx..start_idx + 3]
            }
        };
        iter.filter(|w| !illegle.contains(&w.chars().next().unwrap()))
            .map(|word| {
                let mut new_used = self.used_chars;
                for c in word.chars() {
                    new_used[self.board.get_idx(c)] = true;
                }
                let end_idx = self.board.get_idx(word.chars().last().unwrap());
                State {
                    board: self.board,
                    word,
                    used_chars: new_used,
                    location: Location::Idx(end_idx),
                    path_len: self.path_len + 1,
                }
            })
            .collect()
    }

    fn calculate_score(&self) -> OrderedF32 {
        // The reasoning behind this heuristic is to prioritize paths that use more characters
        // while also considering the length of the path. By dividing by `self.path_len + 1`,
        // it ensures that shorter paths are favored, balancing between path length
        // and character usage.

        OrderedF32::from(self.used_chars.iter().filter(|&&b| b).count())
            / (OrderedF32::from(self.path_len) + 1.0.into())
    }

    fn is_goal(&self) -> bool {
        self.used_chars.iter().all(|&f| f)
    }
}

pub fn solve(board: &Board, trie: &Trie) -> Vec<String> {
    let start = State {
        board,
        word: "".into(),
        path_len: 0,
        used_chars: [false; 12],
        location: Location::Root,
    };
    let mut parent = IndexMap::new();
    let mut queue = BinaryHeap::new();
    let score = start.calculate_score();
    let (idx, _) = parent.insert_full(start, 0);
    queue.push((score, idx));

    while !queue.is_empty() {
        let (_, parent_idx) = queue.pop().unwrap();
        let (parent_state, _) = parent.get_index(parent_idx).unwrap();
        for child in parent_state.get_child_states(trie) {
            let new_score = child.calculate_score();
            if child.is_goal() {
                let (new_idx, _) = parent.insert_full(child, parent_idx);
                return extract_path(parent, new_idx);
            }
            let (new_idx, _) = parent.insert_full(child, parent_idx);
            queue.push((new_score, new_idx));
        }
    }

    vec![]
}

fn extract_path(parent: IndexMap<State, usize>, new_idx: usize) -> Vec<String> {
    let (mut current_state, mut next_state) = parent.get_index(new_idx).unwrap();
    let mut path = vec![];
    while *next_state != 0 {
        path.push(current_state.word.clone());
        (current_state, next_state) = parent.get_index(*next_state).unwrap();
    }
    path.push(current_state.word.clone());
    path.reverse();
    path
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
    // TODO: test extract_path and solve
}
