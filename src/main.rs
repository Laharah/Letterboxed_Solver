use std::collections::HashSet;

#[derive(Debug, Clone, Copy)]
enum Data {
    Letter(char),
    End,
    Root,
}

#[derive(Debug)]
struct Node {
    data: Data,
    children: Vec<usize>,
}

impl Node {
    fn new(data: Data) -> Self {
        Node {
            data,
            children: vec![],
        }
    }
}

const ROOT: usize = 0;
const WORD_END: usize = 1;
/// Trie data structure, root is always at 0, "endword" node is at 1
#[derive(Debug)]
struct Trie {
    nodes: Vec<Node>,
}

impl Trie {
    fn new(word_list: &[String]) -> Self {
        let mut t = Trie {
            nodes: vec![Node::new(Data::Root), Node::new(Data::End)],
        };
        for word in word_list {
            t.insert(word);
        }
        t
    }

    fn contains(&self, word: &str) -> bool {
        let mut cursor = ROOT;
        for c in word.chars() {
            let mut found = false;
            for &idx in &self.nodes[cursor].children {
                match self.nodes[idx].data {
                    Data::Letter(l) if l == c => {
                        cursor = idx;
                        found = true;
                        break;
                    }
                    _ => {}
                }
            }
            if !found {
                return false;
            }
        }
        println!("{}", cursor);
        if self.nodes[cursor].children.contains(&WORD_END) {
            return true;
        }

        false
    }

    fn insert(&mut self, word: &str) {
        let mut cursor = ROOT;
        for c in word.chars() {
            let mut found = false;
            for idx in &self.nodes[cursor].children {
                if let Data::Letter(_) = self.nodes[*idx].data {
                    cursor = *idx;
                    found = true;
                    break;
                }
            }
            if !found {
                self.nodes.push(Node {
                    data: Data::Letter(c),
                    children: Vec::new(),
                });
                let idx = self.nodes.len() - 1;
                self.nodes[cursor].children.push(idx);
                cursor = idx;
            }
        }
        self.nodes[cursor].children.push(WORD_END);
    }
}

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
    println!("{:?}", t.nodes);
    assert!(t.contains("test"));
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn trie_store() {
        let t = Trie::new(&[String::from("test")]);
        println!("{:?}", t.nodes);
        assert!(t.contains("test"));
    }

    #[test]
    fn trie_does_not_contain() {
        let t = Trie::new(&[String::from("test")]);
        println!("{:?}", t.nodes);
        assert!(!t.contains("tested"));
        assert!(!t.contains("tesg"));
        assert!(!t.contains("nothere"));
    }

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
