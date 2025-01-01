fn main() {}

struct Solution;

impl Solution {
    pub fn majority_element(nums: Vec<i32>) -> Vec<i32> {
        let mut candidate1 = i32::MIN;
        let mut candidate2 = i32::MIN;
        let mut count1 = 0;
        let mut count2 = 0;
        for &num in nums.iter() {
            if candidate1 == num {
                count1 += 1;
            } else if candidate2 == num {
                count2 += 1;
            } else if count1 == 0 {
                candidate1 = num;
                count1 += 1;
            } else if count2 == 0 {
                candidate2 = num;
                count2 += 1;
            } else {
                count1 -= 1;
                count2 -= 1;
            }
        }
        count1 = 0;
        count2 = 0;
        for &num in nums.iter() {
            if candidate1 == num {
                count1 += 1;
            }
            if candidate2 == num {
                count2 += 1;
            }
        }
        let mut ans = vec![];
        if count1 > nums.len() / 3 {
            ans.push(candidate1);
        }
        if candidate1 != candidate2 && count2 > nums.len() / 3 {
            ans.push(candidate2);
        }
        ans
    }
}
