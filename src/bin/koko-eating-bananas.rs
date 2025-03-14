fn main() {
    assert_eq!(4, Solution::min_eating_speed(vec![3, 6, 7, 11], 8));
    assert_eq!(30, Solution::min_eating_speed(vec![30, 11, 23, 4, 20], 5));
    assert_eq!(23, Solution::min_eating_speed(vec![30, 11, 23, 4, 20], 6));
    assert_eq!(1, Solution::min_eating_speed(vec![312884470], 968709470));
    assert_eq!(1, Solution::min_eating_speed(vec![4], 4));
    assert_eq!(
        3,
        Solution::min_eating_speed(vec![805306368, 805306368, 805306368], 1000000000),
    );
}

struct Solution;

impl Solution {
    pub fn min_eating_speed(piles: Vec<i32>, h: i32) -> i32 {
        let h = h as i64;
        let mut low = 0i64;
        let mut high = *piles.iter().max().unwrap() as i64;
        while low < high {
            let mid = (low + high + 1) >> 1;
            let time = piles
                .iter()
                .fold(0i64, |c, &p| c + (p as i64 + mid - 1) / mid);
            if time > h {
                low = mid;
            } else {
                high = mid - 1;
            }
        }
        (low + 1) as i32
    }
}
