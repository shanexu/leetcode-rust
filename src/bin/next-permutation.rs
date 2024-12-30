fn main() {
    let mut vec = vec![2, 2, 1, 1];
    Solution::next_permutation(&mut vec);
    println!("{:?}", vec);
}

struct Solution {}

impl Solution {
    pub fn next_permutation(nums: &mut Vec<i32>) {
        if nums.len() == 1 {
            return;
        }
        if nums.len() == 2 {
            let t = nums[0];
            nums[0] = nums[1];
            nums[1] = t;
            return;
        }
        for i in 1..nums.len() {
            let j = nums.len() - i;
            swap(nums, j - 1, j);
            if nums[j - 1] > nums[j] {
                break;
            }
            let mut k = nums.len() - 1;
            for t in j..(nums.len() - 1) {
                if nums[t] < nums[t + 1] {
                    k = t;
                    break;
                } else {
                    swap(nums, t, t + 1);
                }
            }
            if k != nums.len() - 1 {
                let t = nums[k + 1];
                for q in (j - 1)..(k + 1) {
                    let p = k + j - q - 1;
                    nums[p + 1] = nums[p];
                }
                nums[j - 1] = t;
                break;
            }
        }
    }
}

fn swap(nums: &mut Vec<i32>, i: usize, j: usize) {
    let t = nums[i];
    nums[i] = nums[j];
    nums[j] = t;
}
