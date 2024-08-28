pub const BOARD_LEN: usize = 12;
pub const SIDE_LEN: usize = BOARD_LEN / 4;

/// Struct to represent the letterboxed board
#[derive(Hash, Eq, PartialEq, Debug)]
pub struct Board {
    pub letters: [char; BOARD_LEN],
}

impl Board {
    /// Display the board in a human-readable console format
    pub fn show(&self) {
        print!("  ");
        for c in self.letters[..SIDE_LEN].iter() {
            print!("  {} ", c.to_ascii_uppercase());
        }
        print!(" ");
        let gap_len = BOARD_LEN - 1;
        let gap = String::from(" ").repeat(gap_len);
        print!("\n  ┌");
        for i in 0usize..SIDE_LEN {
            if i != 2 {
                print!("─┬──")
            } else {
                println!("─┬─┐");
            }
        }
        for i in 0usize..SIDE_LEN {
            println!(
                " {}├{}┤{} ",
                self.letters[SIDE_LEN * 3..][2 - i].to_ascii_uppercase(),
                gap,
                self.letters[SIDE_LEN..SIDE_LEN * 2][i].to_ascii_uppercase()
            );
            if i != 2 {
                println!("  │{}│  ", gap);
            }
        }
        print!("  └");
        for i in 0usize..SIDE_LEN {
            if i != 2 {
                print!("─┴──")
            } else {
                println!("─┴─┘");
            }
        }
        print!("   ");
        for c in self.letters[SIDE_LEN * 2..SIDE_LEN * 3].iter().rev() {
            print!(" {}  ", c.to_ascii_uppercase());
        }
        print!("\n\n");
    }
    /// get the index of a character in the board
    pub fn get_idx(&self, c: char) -> usize {
        // we iterate because 12 items is fast enough compared to a hashmap.
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
        let mut letters = [' '; BOARD_LEN];
        let mut count: usize = 0;

        for c in input {
            if c == ' ' {
                continue;
            }
            letters[count] = c;
            count += 1;
        }

        if count != BOARD_LEN {
            panic!("Invalid board");
        }

        Board { letters }
    }
}

#[cfg(test)]
mod test {
    use core::f32;

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
    fn board_is_square() {
        assert!((BOARD_LEN as f32 / SIDE_LEN as f32) - 4.0 <= f32::EPSILON);
        println!("{}", f32::EPSILON);
    }
}
