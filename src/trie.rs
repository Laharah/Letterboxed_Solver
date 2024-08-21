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
    descendants_count: usize,
    min_depth: usize,
    children: Vec<usize>,
    parent: usize,
}

impl Node {
    fn new(data: Data) -> Self {
        Node {
            data,
            children: vec![],
            min_depth: 0,
            descendants_count: 0,
            parent: 0,
        }
    }
}

/// Trie data structure. Root is always at 0, "end word" node is always at 1
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
        let mut trie = Trie {
            nodes: vec![Node::new(Data::Root), Node::new(Data::End)],
            items: 0,
        };
        for word in word_list {
            trie.insert(word.as_ref());
        }
        trie
    }

    pub fn new_with_board<T, I>(word_list: T, board: &Board) -> Self
    where
        T: IntoIterator<Item = I>,
        I: AsRef<str>,
    {
        let mut trie = Trie {
            nodes: vec![Node::new(Data::Root), Node::new(Data::End)],
            items: 0,
        };
        for word in word_list {
            if legal_word(&word, board) {
                trie.insert(word.as_ref())
            }
        }
        trie
    }

    pub fn len(&self) -> usize {
        self.items
    }

    pub fn contains(&self, word: &str) -> bool {
        let mut current_node_index = ROOT;

        // Iterate over each character in the word
        for character in word.chars() {
            let mut character_found = false;

            // Check each child of the current node
            for &child_index in &self.nodes[current_node_index].children {
                match self.nodes[child_index].data {
                    // If the child node contains the character, move to that node
                    Data::Letter(letter) if letter == character => {
                        current_node_index = child_index;
                        character_found = true;
                        break;
                    }
                    _ => {}
                }
            }

            // If the character was not found among the children, the word is not in the trie
            if !character_found {
                return false;
            }
        }

        // Check if the current node has a WORD_END child, indicating the end of a valid word
        self.nodes[current_node_index].children.contains(&WORD_END)
    }

    pub fn insert(&mut self, word: &str) {
        let mut path = vec![ROOT];
        let mut current_node_index = ROOT;

        // Iterate over each character in the word
        for (char_index, character) in word.chars().enumerate() {
            let mut character_found = false;

            // Check each child of the current node
            for &child_index in &self.nodes[current_node_index].children {
                if self.nodes[child_index].data == Data::Letter(character) {
                    current_node_index = child_index;
                    path.push(current_node_index);
                    character_found = true;
                    break;
                }
            }

            // If the character was not found among the children, create a new node
            if !character_found {
                self.nodes.push(Node {
                    data: Data::Letter(character),
                    children: Vec::new(),
                    min_depth: word.len() - char_index,
                    descendants_count: 0,
                    parent: current_node_index,
                });
                let new_node_index = self.nodes.len() - 1;
                self.nodes[current_node_index].children.push(new_node_index);
                current_node_index = new_node_index;
                path.push(current_node_index);
            }
        }

        // Add WORD_END marker if it doesn't already exist
        if !self.nodes[current_node_index].children.contains(&WORD_END) {
            self.nodes[current_node_index].children.push(WORD_END);
            self.items += 1;
        }

        // Update min_depth and descendants_count for each node in the path
        let path_len = path.len();
        for (current_depth, node_index) in path.into_iter().enumerate() {
            if node_index != WORD_END {
                let updated_depth = self.nodes[node_index]
                    .min_depth
                    .min(path_len - current_depth);
                self.nodes[node_index].min_depth = updated_depth;
            }
            self.nodes[node_index].descendants_count += 1;
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
        //sort by
        let reverse_depth_sort = |&idx: &usize| Reverse(self.nodes[idx].min_depth);
        root_children.sort_by_key(reverse_depth_sort);
        let first_node_idx = root_children.pop().unwrap();
        let stack_base = (ROOT, root_children);
        let mut first_children = self.nodes[first_node_idx].children.clone();
        first_children.sort_by_key(reverse_depth_sort);
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
        let reverse_depth_sort = |&idx: &usize| Reverse(self.nodes[idx].min_depth);
        let mut head_children = self.nodes[node_idx].children.clone();
        head_children.sort_by_key(reverse_depth_sort);
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

fn legal_word<S>(word: S, board: &Board) -> bool
where
    S: AsRef<str>,
{
    let mut prev_char = None;
    for c in word.as_ref().chars() {
        if !board.letters.contains(&c) {
            return false;
        }
        match prev_char {
            Some(p) if board.get_idx(p) / 3 == board.get_idx(c) / 3 => return false,
            _ => (),
        };
        prev_char = Some(c);
    }
    true
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
            // println!("\nstack:");
            // for (idx, children) in self.stack.iter() {
            //     let c = self.trie.nodes[*idx].data;
            //     let depth = self.trie.nodes[*idx].min_depth;
            //     let kids = children
            //         .iter()
            //         .map(|&idx| match self.trie.nodes[idx].data {
            //             Data::Letter(c) => (idx, c, format!("Depth: {}", depth)),
            //             Data::End => (idx, '_', format!("Depth: {}", depth)),
            //             Data::Root => (idx, 'R', format!("Depth: {}", depth)),
            //         })
            //         .collect::<Vec<_>>();
            //     println!("{:?}: {:?}", c, kids);
            // }

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
            let reverse_depth_sort = |&idx: &usize| Reverse(self.trie.nodes[idx].min_depth);
            next_children.sort_by_key(reverse_depth_sort);
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
            t.nodes[node].descendants_count
        };

        assert_eq!(t.nodes[ROOT].descendants_count, 3);
        assert_eq!(get_descendents_from_prefix("tes"), 2);
        assert_eq!(get_descendents_from_prefix("testi"), 1);
        assert_eq!(get_descendents_from_prefix("qui"), 1);
    }
    #[test]
    fn shortest_word_first_iterator() {
        let mut t = Trie::new(["tests"]);
        t.insert("testr"); // Sort should be stable, **NOT A GUARANTEE**
        t.insert("testing");
        t.insert("quiet");
        t.insert("quietly");
        t.insert("v.q");
        t.insert("very_quietly");
        println!("{:#?}", t.iter().collect::<Vec<_>>());

        let mut iter = t.iter();
        assert_eq!(iter.next().unwrap(), "v.q");
        assert_eq!(iter.next().unwrap(), "very_quietly");
        assert_eq!(iter.next().unwrap(), "quiet");
        assert_eq!(iter.next().unwrap(), "quietly");
        assert_eq!(iter.next().unwrap(), "testr");
        assert_eq!(iter.next().unwrap(), "tests");
        assert_eq!(iter.next().unwrap(), "testing");
    }

    #[test]
    fn iter_from_test() {
        let t = Trie::new(["apple", "app", "apricot", "banana", "band", "bandana"]);
        let mut iter = t.iter_from("app");
        let expected = ["apple", "app"];
        assert!(expected.contains(&iter.next().unwrap().as_str()));
        assert!(expected.contains(&iter.next().unwrap().as_str()));
        assert!(iter.next().is_none());

        let mut iter = t.iter_from("ban");
        let expected = ["bandana", "band", "banana"];
        assert!(expected.contains(&iter.next().unwrap().as_str()));
        assert!(expected.contains(&iter.next().unwrap().as_str()));
        assert!(expected.contains(&iter.next().unwrap().as_str()));
        assert!(iter.next().is_none());

        let mut iter = t.iter_from("band");

        assert!(expected.contains(&iter.next().unwrap().as_str()));
        assert!(expected.contains(&iter.next().unwrap().as_str()));
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
