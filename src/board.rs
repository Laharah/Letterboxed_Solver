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
