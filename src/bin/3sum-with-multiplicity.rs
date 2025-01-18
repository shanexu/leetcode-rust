fn main() {
    println!(
        "{:?}",
        Solution::three_sum_multi(vec![1, 1, 2, 2, 3, 3, 4, 4, 5, 5], 8)
    );
}

struct Solution;

impl Solution {
    pub fn three_sum_multi(arr: Vec<i32>, target: i32) -> i32 {
        let target = target as usize;
        let mut freq = vec![0; 101];
        for x in arr {
            freq[x as usize] += 1;
        }
        let mut ans: i64 = 0;
        for i in 0..=100 {
            if freq[i] == 0 {
                continue;
            }
            freq[i] -= 1;
            for j in i..=100 {
                let mut c = freq[i] + 1;
                if freq[j] == 0 {
                    continue;
                }
                c *= freq[j];
                freq[j] -= 1;
                if target >= 2 * j + i {
                    let k = target - i - j;
                    if k <= 100 && freq[k] != 0 {
                        c *= freq[k];
                        if i == j && j == k {
                            c /= 6;
                        } else if i == j || j == k {
                            c /= 2;
                        }
                        ans += c;
                    }
                }
                freq[j] += 1;
            }
            freq[i] += 1;
        }
        (ans % MOD) as i32
    }
}

const MOD: i64 = 1_000_000_007;
