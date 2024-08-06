#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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

/// Trie data structure, root is always at 0, "endword" node is at 1
const ROOT: usize = 0;
const WORD_END: usize = 1;
#[derive(Debug)]
pub struct Trie {
    nodes: Vec<Node>,
    items: usize,
}
impl Trie {
    pub fn new<T, I>(word_list: T) -> Self
    where
        T: IntoIterator<Item = I>,
        I: AsRef<str>,
    {
        let mut t = Trie {
            nodes: vec![Node::new(Data::Root), Node::new(Data::End)],
            items: 0,
        };
        for word in word_list {
            t.insert(word.as_ref());
        }
        t
    }

    pub fn len(&self) -> usize {
        self.items
    }

    pub fn contains(&self, word: &str) -> bool {
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
                if self.nodes[*idx].data == Data::Letter(c) {
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

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn trie_store() {
        let t = Trie::new(["test"]);
        println!("{:?}", t.nodes);
        assert!(t.contains("test"));
    }

    #[test]
    fn trie_contains() {
        let mut t = Trie::new(["test"]);
        t.insert("testing");
        t.insert("quick");
        println!("{:?}", t.nodes);
        assert!(t.contains("test"));
        assert!(t.contains("testing"));
        assert!(t.contains("quick"));
    }

    #[test]
    fn trie_does_not_contain() {
        let t = Trie::new(["test"]);
        println!("{:?}", t.nodes);
        assert!(!t.contains("tested"));
        assert!(!t.contains("tesg"));
        assert!(!t.contains("nothere"));
    }
}
