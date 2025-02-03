fn main() {
    assert_eq!(
        Solution::num_equiv_domino_pairs(vec![vec![1, 2], vec![2, 1], vec![3, 4], vec![5, 6]]),
        1
    );
}

struct Solution;

impl Solution {
    pub fn num_equiv_domino_pairs(dominoes: Vec<Vec<i32>>) -> i32 {
        let mut table = vec![0; 100];
        for xs in dominoes {
            let mut x1 = xs[0];
            let mut x2 = xs[1];
            if x1 > x2 {
                std::mem::swap(&mut x1, &mut x2);
            }
            table[(x1 * 10 + x2) as usize] += 1;
        }
        table.iter().map(|c| c * (c - 1) / 2).sum()
    }
}
