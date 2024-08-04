mod trie;
use trie::Trie;

struct Board {
    letters: [[char; 3]; 4],
}
impl Board {
    fn new(letters: &str) -> Self {
        if letters.len() < 12 {
            panic!("Invalid board");
        }
        let mut edges = [[' '; 3]; 4];
        let mut i = 0;
        let mut j = 0;

        for c in letters.chars() {
            if c == ' ' {
                continue;
            }
            edges[i][j] = c;
            j += 1;
            if j == 3 {
                j = 0;
                i += 1;
            }
        }
        if i != 4 && j != 0 {
            panic!("Invalid board");
        } else {
            Board { letters: edges }
        }
    }
}

fn main() {
    let b = Board::new("abcdefghijkl");
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
        Board::new("abcdefghi jkl ");
    }

    #[test]
    fn board_construction_failure() {
        assert!(std::panic::catch_unwind(|| Board::new("abcdefg")).is_err());
        assert!(std::panic::catch_unwind(|| Board::new("abcdefghi")).is_err());
        assert!(std::panic::catch_unwind(|| Board::new("abcdefghij")).is_err());
        assert!(std::panic::catch_unwind(|| Board::new("abcdefghijk")).is_err());
        assert!(std::panic::catch_unwind(|| Board::new("abcdefghijklm")).is_err());
    }
}
