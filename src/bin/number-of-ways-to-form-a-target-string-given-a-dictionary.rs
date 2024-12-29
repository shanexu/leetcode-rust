fn main() {
    println!(
        "{}",
        Solution::num_ways(
            vec!["acca".to_string(), "bbbb".to_string(), "caca".to_string()],
            "aba".to_string()
        )
    );
}

struct Solution;

impl Solution {
    pub fn num_ways(words: Vec<String>, target: String) -> i32 {
        let target = target.as_bytes();
        let m = target.len();
        let n = words[0].len();
        let mut frequency: Vec<Vec<i32>> = vec![vec![0; 26]; n];
        for w in words.iter() {
            for (j, &c) in w.as_bytes().iter().enumerate() {
                frequency[j][(c - b'a') as usize] += 1;
            }
        }

        let mut memo = vec![vec![-1; n]; m];
        fn helper(
            i: usize,
            j: usize,
            m: usize,
            n: usize,
            target: &[u8],
            frequency: &Vec<Vec<i32>>,
            memo: &mut Vec<Vec<i32>>,
        ) -> i32 {
            if i >= m {
                return 1;
            }
            if j >= n {
                return 0;
            }
            if memo[i][j] != -1 {
                return memo[i][j];
            }
            let mut ans: i64 = helper(i, j + 1, m, n, target, frequency, memo) as i64;
            ans += helper(i + 1, j + 1, m, n, target, frequency, memo) as i64
                * frequency[j][(target[i] - b'a') as usize] as i64;
            ans %= MOD;
            memo[i][j] = ans as i32;
            ans as i32
        }
        helper(0, 0, m, n, target, &frequency, &mut memo)
    }
}

const MOD: i64 = 1_000_000_007;
