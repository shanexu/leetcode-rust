fn main() {}

struct Solution;

impl Solution {
    pub fn next_greater_element(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        let mut stack: Vec<i32> = vec![];
        let mut map = std::collections::HashMap::new();
        for x in nums2 {
            while !stack.is_empty() && stack[stack.len() - 1] < x {
                let v = stack.pop().unwrap();
                map.insert(v, x);
            }
            stack.push(x);
        }
        while let Some(x) = stack.pop() {
            map.insert(x, -1);
        }
        nums1.iter().map(|x| *map.get(x).unwrap()).collect()
    }
}
