fn main() {}

struct Solution;

impl Solution {
    pub fn find_words(mut board: Vec<Vec<char>>, words: Vec<String>) -> Vec<String> {
        let mut trie = Trie::new();
        for (i, word) in words.iter().enumerate() {
            trie.insert(word, i);
        }
        let rows = board.len();
        let cols = board[0].len();
        let mut found_words = Vec::with_capacity(words.len());
        for i in 0..rows {
            for j in 0..cols {
                dfs(i, j, rows, cols, &mut board, &mut trie, &mut found_words);
            }
        }
        found_words.into_iter().map(|i| words[i].clone()).collect()
    }
}

fn dfs(
    i: usize,
    j: usize,
    rows: usize,
    cols: usize,
    board: &mut Vec<Vec<char>>,
    mut node: &mut Trie,
    found_words: &mut Vec<usize>,
) {
    let char_idx = board[i][j] as usize - 'a' as usize;
    if node.children[char_idx].is_none() {
        return;
    }
    node = node.children[char_idx].as_mut().unwrap();
    if let Some(r) = node.reference {
        found_words.push(r);
        node.reference = None;
    }
    let temp_char = board[i][j];
    board[i][j] = '\0';
    for k in 0..4 {
        let new_row = i as i32 + DIRECTIONS[k];
        let new_col = j as i32 + DIRECTIONS[k + 1];
        if new_row >= 0
            && new_row < rows as i32
            && new_col >= 0
            && new_col < cols as i32
            && board[new_row as usize][new_col as usize] != '\0'
        {
            dfs(
                new_row as usize,
                new_col as usize,
                rows,
                cols,
                board,
                node,
                found_words,
            );
        }
    }
    board[i][j] = temp_char;
}

const DIRECTIONS: &[i32] = &[-1, 0, 1, 0, -1];

struct Trie {
    children: [Option<Box<Trie>>; 26],
    reference: Option<usize>,
}

const DEFAULT_TRIE_VALUE: Option<Box<Trie>> = None;
impl Trie {
    fn new() -> Self {
        Trie {
            children: [DEFAULT_TRIE_VALUE; 26],
            reference: None,
        }
    }

    fn insert(&mut self, word: &str, reference: usize) {
        let mut node = self;

        for &character in word.as_bytes() {
            let character_index = (character - b'a') as usize;

            if node.children[character_index].is_none() {
                node.children[character_index] = Some(Box::new(Trie::new()));
            }

            node = node.children[character_index].as_deref_mut().unwrap();
        }

        node.reference = Some(reference);
    }
}
