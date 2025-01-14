fn main() {
    println!(
        "{}",
        Solution::count_characters(
            vec![
                "cat".to_string(),
                "bt".to_string(),
                "hat".to_string(),
                "tree".to_string()
            ],
            "atach".to_string()
        )
    );
}

struct Solution;

impl Solution {
    pub fn count_characters(words: Vec<String>, chars: String) -> i32 {
        let mut ans = 0;
        let c = freq(chars.as_bytes());
        'out: for w in words {
            let w = w.as_bytes();
            let f = freq(w);
            for i in 0..26 {
                if c[i] < f[i] {
                    continue 'out;
                }
            }
            ans += w.len() as i32;
        }
        ans
    }
}

#[inline(always)]
fn freq(word: &[u8]) -> [i32; 26] {
    let mut f = [0; 26];
    for &b in word {
        f[(b - b'a') as usize] += 1;
    }
    f
}
