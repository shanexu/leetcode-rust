fn main() {
    assert_eq!(vec![2, 3, 1], Solution::result_array(vec![2, 1, 3]))
}

struct Solution;

impl Solution {
    pub fn result_array(nums: Vec<i32>) -> Vec<i32> {
        let mut arr1 = vec![nums[0]];
        let mut arr2 = vec![nums[1]];
        for i in 2..nums.len() {
            if arr1[arr1.len() - 1] > arr2[arr2.len() - 1] {
                arr1.push(nums[i]);
            } else {
                arr2.push(nums[i]);
            }
        }
        arr1.extend(arr2);
        arr1
    }
}
