fn main() {
    let board = vec![
        vec!['o', 'a', 'a', 'n'],
        vec!['e', 't', 'a', 'e'],
        vec!['i', 'h', 'k', 'r'],
        vec!['i', 'f', 'l', 'v'],
    ];
    let words = vec![
        "oath".to_string(),
        "pea".to_string(),
        "eat".to_string(),
        "rain".to_string(),
    ];
    println!("{:?}", Solution::find_words(board, words));
}

struct Solution;

impl Solution {
    pub fn find_words(mut board: Vec<Vec<char>>, words: Vec<String>) -> Vec<String> {
        let rows = board.len();
        let cols = board[0].len();
        let mut board_freq = vec![0; 26];
        for i in 0..rows {
            for j in 0..cols {
                board_freq[board[i][j] as usize - 'a' as usize] += 1;
            }
        }
        let mut trie = Trie::new();
        'out: for word in words {
            let mut board_freq = board_freq.clone();
            let bs = word.as_bytes();
            for &b in bs {
                if board_freq[b as usize - 'a' as usize] == 0 {
                    continue 'out;
                }
                board_freq[b as usize - 'a' as usize] -= 1;
            }
            let reverse = board_freq[bs[0] as usize - 'a' as usize]
                > board_freq[bs[bs.len() - 1] as usize - 'a' as usize];
            trie.insert(word, reverse);
        }
        let mut found_words = vec![];
        for i in 0..rows {
            for j in 0..cols {
                dfs(i, j, rows, cols, &mut board, &mut trie, &mut found_words);
            }
        }
        found_words
    }
}

#[inline(always)]
fn dfs(
    i: usize,
    j: usize,
    rows: usize,
    cols: usize,
    board: &mut Vec<Vec<char>>,
    tire: &mut Trie,
    found_words: &mut Vec<String>,
) {
    let char_idx = board[i][j] as usize - 'a' as usize;
    if tire.children[char_idx].is_none() {
        return;
    }
    let node = tire.children[char_idx].as_mut().unwrap();
    if let Some(w) = node.word.take() {
        found_words.push(w);
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
    if node.children.iter().all(|x| x.is_none()) {
        tire.children[char_idx] = None;
    }
}

const DIRECTIONS: &[i32] = &[-1, 0, 1, 0, -1];

struct Trie {
    children: [Option<Box<Trie>>; 26],
    word: Option<String>,
}

const DEFAULT_TRIE_VALUE: Option<Box<Trie>> = None;
impl Trie {
    #[inline(always)]
    fn new() -> Self {
        Self {
            children: [DEFAULT_TRIE_VALUE; 26],
            word: None,
        }
    }

    #[inline(always)]
    fn insert(&mut self, word: String, reverse: bool) {
        let mut node = self;
        if reverse {
            for &ch in word.as_bytes().iter().rev() {
                let ch_idx = (ch - b'a') as usize;
                if node.children[ch_idx].is_none() {
                    node.children[ch_idx] = Some(Box::new(Trie::new()));
                }
                node = node.children[ch_idx].as_deref_mut().unwrap();
            }
        } else {
            for &ch in word.as_bytes() {
                let ch_idx = (ch - b'a') as usize;
                if node.children[ch_idx].is_none() {
                    node.children[ch_idx] = Some(Box::new(Trie::new()));
                }
                node = node.children[ch_idx].as_deref_mut().unwrap();
            }
        }
        node.word = Some(word);
    }
}
