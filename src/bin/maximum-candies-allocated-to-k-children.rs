fn main() {
    assert_eq!(5, Solution::maximum_candies(vec![5, 8, 6], 3));
    assert_eq!(0, Solution::maximum_candies(vec![2, 5], 11));
}

struct Solution;

impl Solution {
    pub fn maximum_candies(mut candies: Vec<i32>, k: i64) -> i32 {
        let mut low = 0;
        candies.sort();
        candies.reverse();
        let mut high = candies[0];
        'outter: while low < high {
            let mid = (low + high + 1) / 2;
            let mut count = 0i64;
            for &candy in candies.iter() {
                count += (candy / mid) as i64;
                if count >= k {
                    low = mid;
                    continue 'outter;
                }
            }
            high = mid - 1;
        }
        low
    }
}
