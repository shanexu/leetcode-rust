fn main() {
    println!("{}", Solution::repair_cars(vec![4, 3, 2, 1], 10));
    println!("{}", Solution::repair_cars(vec![5, 1, 8], 6));
    println!("{}", Solution::repair_cars(vec![1, 1, 3, 3], 74));
}

struct Solution;

impl Solution {
    pub fn repair_cars(ranks: Vec<i32>, cars: i32) -> i64 {
        let cars = cars as i64;
        let mut right = *ranks.iter().max().unwrap() as i64 * cars * cars;
        let mut left: i64 = 0;
        'out: while left <= right {
            let mid = (left + right) >> 1;
            let mut count = 0;
            for &r in ranks.iter() {
                let r = r as i64;
                count += ((mid / r) as f64).sqrt() as i64;
                if count >= cars {
                    right = mid - 1;
                    continue 'out;
                }
            }
            left = mid + 1;
        }
        left
    }
}
