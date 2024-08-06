mod trie;
use std::io::BufRead;

use trie::Trie;

struct Board {
    letters: [[char; 3]; 4],
}

impl<T> From<T> for Board
where
    T: IntoIterator<Item = char>,
{
    fn from(input: T) -> Self {
        let mut edges = [[' '; 3]; 4];
        let mut i = 0;
        let mut j = 0;
        let mut count = 0;

        for c in input {
            if c == ' ' {
                continue;
            }
            edges[i][j] = c;
            j += 1;
            if j == 3 {
                j = 0;
                i += 1;
            }
            count += 1;
        }

        if count != 12 {
            panic!("Invalid board");
        }

        Board { letters: edges }
    }
}

fn main() {
    let f = std::fs::File::open("/usr/share/dict/american-english").unwrap();
    let f = std::io::BufReader::new(f);
    let filtered = f
        .lines()
        .map(|l| l.unwrap())
        .filter(|l| {
            // let l = l.unwrap();
            if l.ends_with("'s") {
                return false;
            }
            true
        })
        .map(|l| l.trim().to_lowercase());

    let t = Trie::new(filtered);
    // println!("{:?}", t);
    // let b = Board::from("abcdefghijkl".chars());
    // println!("{:?}", b.letters);
    assert!(t.contains("test"));
    assert!(t.contains("points"));
    assert!(t.contains("coding"));
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
