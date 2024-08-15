use crate::game::Board;
use std::cmp::Reverse;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Data {
    Letter(char),
    End,
    Root,
}

#[derive(Debug)]
struct Node {
    data: Data,
    descendents_count: usize,
    children: Vec<usize>,
    parent: usize,
}

impl Node {
    fn new(data: Data) -> Self {
        Node {
            data,
            children: vec![],
            descendents_count: 0,
            parent: 0,
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

    pub fn new_with_board<T, I>(word_list: T, board: &Board) -> Self
    where
        T: IntoIterator<Item = I>,
        I: AsRef<str>,
    {
        let mut t = Trie {
            nodes: vec![Node::new(Data::Root), Node::new(Data::End)],
            items: 0,
        };
        let legal_word = |word: &I| -> bool {
            let prev_char = None;
            for c in word.as_ref().chars() {
                if !board.letters.contains(&c) {
                    return false;
                }
                match prev_char {
                    Some(p) if board.get_idx(p) / 3 == board.get_idx(c) / 3 => return false,
                    _ => (),
                };
            }
            true
        };
        for w in word_list {
            if legal_word(&w) {
                t.insert(w.as_ref())
            }
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

    pub fn insert(&mut self, word: &str) {
        let mut path = vec![ROOT];
        let mut cursor = ROOT;
        for c in word.chars() {
            let mut found = false;
            for idx in &self.nodes[cursor].children {
                if self.nodes[*idx].data == Data::Letter(c) {
                    cursor = *idx;
                    path.push(cursor);
                    found = true;
                    break;
                }
            }
            if !found {
                self.nodes.push(Node {
                    data: Data::Letter(c),
                    children: Vec::new(),
                    descendents_count: 0,
                    parent: cursor,
                });
                let idx = self.nodes.len() - 1;
                self.nodes[cursor].children.push(idx);
                cursor = idx;
                path.push(cursor);
            }
        }
        if !self.nodes[cursor].children.contains(&WORD_END) {
            self.nodes[cursor].children.push(WORD_END);
            self.items += 1;
        }
        for p in path {
            self.nodes[p].descendents_count += 1;
        }
    }

    fn get_node_from_prefix(&self, prefix: &str) -> Option<usize> {
        let mut cursor = ROOT;
        for c in prefix.chars() {
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
                return None;
            }
        }
        Some(cursor)
    }

    pub fn iter(&self) -> TrieIterator {
        let mut root_children = self.nodes[ROOT].children.clone();
        let descendent_sort = |&idx: &usize| Reverse(self.nodes[idx].descendents_count);
        root_children.sort_by_key(descendent_sort);
        let first_node_idx = root_children.pop().unwrap();
        let stack_base = (ROOT, root_children);
        let mut first_children = self.nodes[first_node_idx].children.clone();
        first_children.sort_by_key(descendent_sort);
        let stack_head = (first_node_idx, first_children);
        TrieIterator {
            trie: self,
            stack: vec![stack_base, stack_head],
        }
    }

    pub fn iter_from(&self, prefix: &str) -> TrieIterator {
        let mut node_idx = match self.get_node_from_prefix(prefix) {
            Some(x) => x,
            None => {
                return TrieIterator {
                    trie: self,
                    stack: vec![],
                }
            }
        };
        let descendent_sort = |&idx: &usize| self.nodes[idx].descendents_count;
        let mut head_children = self.nodes[node_idx].children.clone();
        head_children.sort_by_key(descendent_sort);
        let mut stack = vec![(node_idx, head_children)];
        node_idx = self.nodes[node_idx].parent;
        while node_idx != ROOT {
            stack.push((node_idx, vec![]));
            node_idx = self.nodes[node_idx].parent;
        }
        stack.push((ROOT, vec![]));
        stack.reverse();
        TrieIterator { trie: self, stack }
    }
}

#[derive(Debug)]
pub struct TrieIterator<'a> {
    trie: &'a Trie,
    // node_idx, vec of shildren
    stack: Vec<(usize, Vec<usize>)>,
}

impl Iterator for TrieIterator<'_> {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            // // debug show stack
            // for (idx, children) in self.stack.iter() {
            //     let c = self.trie.nodes[*idx].data;
            //     let kids = children
            //         .iter()
            //         .map(|&idx| match self.trie.nodes[idx].data {
            //             Data::Letter(c) => (idx, c),
            //             Data::End => (idx, '_'),
            //             Data::Root => (idx, 'R'),
            //         })
            //         .collect::<Vec<_>>();
            //     println!("{:?}: {:?}", c, kids);
            // }
            // println!("stack:");
            // ---

            let (node_idx, mut children) = match self.stack.pop() {
                None => return None,
                Some(x) => x,
            };
            let current_child_idx = children.pop().unwrap();

            self.stack.push((node_idx, children));

            if current_child_idx == WORD_END {
                let mut word = String::new();
                let iter = self.stack.iter();
                for (idx, _) in iter {
                    let n = &self.trie.nodes[*idx];
                    if let Data::Letter(c) = n.data {
                        word.push(c);
                    }
                }

                let mut to_del = 0;
                for (_, cursor_descendents) in self.stack.iter().rev() {
                    if !cursor_descendents.is_empty() {
                        break;
                    }
                    to_del += 1;
                }
                self.stack.resize(self.stack.len() - to_del, (0, vec![]));
                return Some(word);
            }

            let mut next_children = self.trie.nodes[current_child_idx].children.clone();
            let descendent_sort = |&idx: &usize| self.trie.nodes[idx].descendents_count;
            next_children.sort_by_key(descendent_sort);
            self.stack.push((current_child_idx, next_children));
        }
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
    fn double_insert() {
        let mut t = Trie::new(["test"]);
        t.insert("test");
        println!("{:#?}", t.nodes);
        assert_eq!(t.len(), 1);
    }

    #[test]
    fn trie_does_not_contain() {
        let t = Trie::new(["test"]);
        println!("{:?}", t.nodes);
        assert!(!t.contains("tested"));
        assert!(!t.contains("tesg"));
        assert!(!t.contains("nothere"));
    }
    #[test]
    fn length() {
        let mut t = Trie::new(["test"]);
        t.insert("testing");
        t.insert("quick");
        assert_eq!(t.len(), 3);
    }
    #[test]
    fn descendents_count() {
        let mut t = Trie::new(["test"]);
        t.insert("testing");
        t.insert("quick");

        println!("{:?}", t.nodes);

        let get_descendents_from_prefix = |prefix: &str| {
            let node = t.get_node_from_prefix(prefix).unwrap();
            t.nodes[node].descendents_count
        };

        assert_eq!(t.nodes[ROOT].descendents_count, 3);
        assert_eq!(get_descendents_from_prefix("tes"), 2);
        assert_eq!(get_descendents_from_prefix("testi"), 1);
        assert_eq!(get_descendents_from_prefix("qui"), 1);
    }
    #[test]
    fn greatest_first_iterator() {
        let mut t = Trie::new(["tests"]);
        t.insert("testr"); // Sort should be stable, **NOT A GUARANTEE**
        t.insert("testing");
        t.insert("quiet");
        t.insert("quietly");
        t.insert("very_quietly");
        let mut iter = t.iter();
        assert_eq!(iter.next().unwrap(), "very_quietly");
        assert_eq!(iter.next().unwrap(), "quietly");
        assert_eq!(iter.next().unwrap(), "quiet");
        assert_eq!(iter.next().unwrap(), "testing");
        assert_eq!(iter.next().unwrap(), "testr");
        assert_eq!(iter.next().unwrap(), "tests");
    }

    #[test]
    fn iter_from_test() {
        let mut t = Trie::new(["apple", "app", "apricot", "banana", "band", "bandana"]);
        let mut iter = t.iter_from("app");

        assert_eq!(iter.next().unwrap(), "apple");
        assert_eq!(iter.next().unwrap(), "app");
        assert!(iter.next().is_none());

        let mut iter = t.iter_from("ban");

        assert_eq!(iter.next().unwrap(), "bandana");
        assert_eq!(iter.next().unwrap(), "band");
        assert_eq!(iter.next().unwrap(), "banana");
        assert!(iter.next().is_none());

        let mut iter = t.iter_from("band");

        assert_eq!(iter.next().unwrap(), "bandana");
        assert_eq!(iter.next().unwrap(), "band");
        assert!(iter.next().is_none());

        let mut iter = t.iter_from("z");

        assert!(iter.next().is_none());
    }

    #[test]
    fn test_new_with_board() {
        let board = Board::from("abc def ghi jkl".chars());
        let words = vec!["adg", "bkfg", "agh"];
        let trie = Trie::new_with_board(words, &board);

        assert!(trie.contains("adg"));
        assert!(trie.contains("bkfg"));
        assert!(!trie.contains("ghi"));
    }
}
