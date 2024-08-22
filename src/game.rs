use crate::non_nan::OrderedF32; // solve requires f32's to be orderable'
use crate::trie::Trie;
use indexmap::IndexMap;
use std::collections::BinaryHeap;

/// Struct to represent the letterboxed board
#[derive(Hash, Eq, PartialEq, Debug)]
pub struct Board {
    pub letters: [char; 12],
}

impl Board {
    /// Display the board in a human-readable console format
    pub fn show(&self) {
        print!("  ");
        for c in self.letters[..3].iter() {
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
    /// get the index of a character in the board
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
    /// construct a board from an iterator of characters, ignores spaces.
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

/// the location on the board that the current state is located.
#[derive(Hash, Eq, PartialEq, Debug, Clone)]
enum Location {
    Root,
    Idx(usize),
}

/// Game state representation used in the A* search algorithm
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
            Some(c) => trie.iter_from_prefix(&c.to_string()),
            None => trie.iter(),
        };
        let start_char = self.word.chars().last();

        // check if the next word is a legal letterboxed word
        //  - Word starts with the last character of the current word
        //  - Word starts with a character that is not on the same side of the board as the last character
        let illegal = match start_char {
            None => &self.board.letters[0..0],
            Some(c) => {
                let start_idx = self.board.get_idx(c) / 3;
                &self.board.letters[start_idx..start_idx + 3]
            }
        };

        iter.filter(|w| !illegal.contains(&w.chars().next().unwrap()))
            .map(|word| {
                let mut new_used_character_count = self.used_chars;
                for c in word.chars() {
                    new_used_character_count[self.board.get_idx(c)] = true;
                }
                let last_character_location = self.board.get_idx(word.chars().last().unwrap());
                State {
                    board: self.board,
                    word,
                    used_chars: new_used_character_count,
                    location: Location::Idx(last_character_location),
                    path_len: self.path_len + 1,
                }
            })
            .collect()
    }

    fn calculate_score(&self) -> OrderedF32 {
        // The reasoning behind this heuristic is to prioritize paths that consume more characters.
        // By dividing by `self.path_len` and `word.len()`, it ensures that shorter paths are
        // favored, balancing between path length and character consumption.(The +1 in the
        // denominator is to avoid division by zero)

        let f = (self.used_chars.iter().filter(|&&b| b).count() as f32)
            / (self.path_len + 1 + self.word.len()) as f32;
        OrderedF32(f)
    }

    /// check if the current state is the target solution
    fn is_goal(&self) -> bool {
        self.used_chars.iter().all(|&f| f)
    }
}

/// Solve the letterboxed game with a given board and word list using the A* search algorithm,
/// prioritizing efficiency
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
        let (_, parent_state_idx) = queue.pop().unwrap();
        let (parent_state, _) = parent.get_index(parent_state_idx).unwrap();
        for child_state in parent_state.get_child_states(trie) {
            let child_score = child_state.calculate_score();
            if child_state.is_goal() {
                let (child_state_idx, _) = parent.insert_full(child_state, parent_state_idx);
                return extract_path(parent, child_state_idx);
            }
            let (new_idx, _) = parent.insert_full(child_state, parent_state_idx);
            queue.push((child_score, new_idx));
        }
    }

    vec![]
}

/// extract the full solution from a given game state
fn extract_path(parent: IndexMap<State, usize>, new_idx: usize) -> Vec<String> {
    let (mut cursor_state, mut next_state) = parent.get_index(new_idx).unwrap();
    let mut path = vec![];
    while *next_state != 0 {
        path.push(cursor_state.word.clone());
        (cursor_state, next_state) = parent.get_index(*next_state).unwrap();
    }
    path.push(cursor_state.word.clone());
    path.reverse();
    path
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn construct_board() {
        let b = Board::from("abcdefghi jkl ".chars());
        assert_eq!(
            b,
            Board {
                letters: ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l']
            }
        );
        let b = Board::from(vec!['a'; 12]);
        assert_eq!(b, Board { letters: ['a'; 12] });
    }

    #[test]
    fn board_construction_failure() {
        assert!(std::panic::catch_unwind(|| Board::from("abcdefg".chars())).is_err());
        assert!(std::panic::catch_unwind(|| Board::from("abcdefghi".chars())).is_err());
        assert!(std::panic::catch_unwind(|| Board::from("abcdefghij".chars())).is_err());
        assert!(std::panic::catch_unwind(|| Board::from("abcdefghijk".chars())).is_err());
        assert!(std::panic::catch_unwind(|| Board::from("abcdefghijklm".chars())).is_err());
    }

    #[test]
    fn solve_game() {
        let board = Board::from("vkspyielurao".chars());
        let word_list = vec![
            "evolve",
            "layover",
            "like",
            "lire",
            "overlay",
            "poke",
            "poker",
            "previously",
            "surly",
            "survive",
            "yak",
            "yolk",
        ];
        let trie = Trie::new_with_board(word_list, &board);

        let solution = solve(&board, &trie);
        assert_eq!(solution, vec!["previously", "yak"]);
    }
}
