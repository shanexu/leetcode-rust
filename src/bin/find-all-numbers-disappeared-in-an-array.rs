fn main() {
    println!("{:?}", Solution::find_disappeared_numbers(vec![1, 1]));
    println!(
        "{:?}",
        Solution::find_disappeared_numbers(vec![4, 3, 2, 7, 8, 2, 3, 1])
    );
}

struct Solution;

impl Solution {
    pub fn find_disappeared_numbers(mut nums: Vec<i32>) -> Vec<i32> {
        let n = nums.len();
        let mut ans = vec![];
        for i in 0..n {
            let mut num = nums[i];
            while !(num == (i + 1) as i32 || num == nums[(num - 1) as usize]) {
                nums.swap(i, (num - 1) as usize);
                num = nums[i];
            }
        }
        for (i, &v) in nums.iter().enumerate() {
            if v != (i + 1) as i32 {
                ans.push((i + 1) as i32);
            }
        }
        ans
    }
}

struct Solution2;

impl Solution2 {
    pub fn find_disappeared_numbers(mut nums: Vec<i32>) -> Vec<i32> {
        let mut ans = vec![];
        let n = nums.len();
        for i in 0..n {
            let idx = (nums[i].abs() - 1) as usize;
            if nums[idx] > 0 {
                nums[idx] = -nums[idx];
            }
        }
        for (i, &num) in nums.iter().enumerate() {
            if num > 0 {
                ans.push(i as i32 + 1);
            }
        }
        ans
    }
}
