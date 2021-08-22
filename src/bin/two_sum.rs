use std::collections::HashMap;

fn main() {
    println!("{:?}", Solution::two_sum_naive(vec![2, 7, 11, 15], 9));
    println!("{:?}", Solution::two_sum_naive(vec![3, 2, 4], 6));
    println!("{:?}", Solution::two_sum_naive(vec![3, 3], 6));

    println!("{:?}", Solution::two_sum(vec![2, 7, 11, 15], 9));
    println!("{:?}", Solution::two_sum(vec![3, 2, 4], 6));
    println!("{:?}", Solution::two_sum(vec![3, 3], 6));
}

struct Solution {}

impl Solution {
    pub fn two_sum_naive(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut res: Vec<i32> = vec![-1, -1];
        for i in 0..nums.len() - 1 {
            for j in (i + 1)..nums.len() {
                if nums[i] + nums[j] == target {
                    res[0] = i as i32;
                    res[1] = j as i32;
                }
            }
        }
        return res;
    }

    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut res: Vec<i32> = vec![-1, -1];
        let mut map: HashMap<i32, usize> = HashMap::new();
        for i in 0..nums.len() {
            let k = nums[i];
            let v = map.entry(k).or_insert(i);
            if *v != i && k + k == target {
                res[0] = *v as i32;
                res[1] = i as i32;
                return res;
            }
        }
        for i in 0..nums.len() {
            let x = nums[i];
            let y = target - x;
            if x == y {
                continue;
            }
            match map.get(&y) {
                Some(j) => {
                    res[0] = i as i32;
                    res[1] = *j as i32;
                    return res;
                }
                None => continue,
            }
        }
        return res;
    }
}
