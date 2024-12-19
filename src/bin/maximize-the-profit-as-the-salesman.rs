fn main() {
    println!(
        "{}",
        Solution::maximize_the_profit(5, vec![vec![0, 0, 1], vec![0, 2, 2], vec![1, 3, 2]])
    );

    println!(
        "{}",
        Solution::maximize_the_profit(5, vec![vec![0, 0, 1], vec![0, 2, 10], vec![1, 3, 2]])
    );
}

struct Solution;

impl Solution {
    pub fn maximize_the_profit(n: i32, offers: Vec<Vec<i32>>) -> i32 {
        let n = n as usize;
        let mut offers_map: Vec<Vec<(usize, i32)>> = vec![vec![]; n];
        for o in offers {
            let start = o[0] as usize;
            let end = o[1] as usize;
            let gold = o[2];
            offers_map[end].push((start, gold));
        }
        let mut memo = vec![0; n];
        for i in 0..n {
            let mut max = if i == 0 { 0 } else { memo[i - 1] };
            for &(start, gold) in offers_map[i].iter() {
                let prev = if start == 0 { 0 } else { memo[start - 1] };
                let current = prev + gold;
                if current > max {
                    max = current;
                }
            }
            memo[i] = max;
        }
        memo[n - 1]
    }
}
