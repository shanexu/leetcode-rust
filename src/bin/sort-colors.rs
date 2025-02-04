fn main() {
    let mut nums = vec![2, 0, 2, 1, 1, 0];
    Solution::sort_colors(&mut nums);
    println!("{:?}", nums);
}

struct Solution;

impl Solution {
    pub fn sort_colors(nums: &mut Vec<i32>) {
        let mut counter: Vec<usize> = vec![0; 3];
        for x in nums.iter() {
            counter[*x as usize] += 1;
        }
        let mut i = 0;
        for (color, &count) in counter.iter().enumerate() {
            for k in i..(i + count) {
                nums[k] = color as i32;
            }
            i = i + count;
        }
    }
}
