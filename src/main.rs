mod trie;
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

        if count < 12 || i != 4 || j != 0 {
            panic!("Invalid board");
        }

        Board { letters: edges }
    }
}

fn main() {
    let b = Board::from("abcdefghijkl".chars());
    println!("{:?}", b.letters);
    let t = Trie::new(&[String::from("test")]);
    println!("{:?}", t);
    assert!(t.contains("test"));
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
