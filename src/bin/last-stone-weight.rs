fn main() {
    println!("{}", Solution::last_stone_weight(vec![2,7,4,1,8,1]));
}

struct Solution;

impl Solution {
    pub fn last_stone_weight(stones: Vec<i32>) -> i32 {
        let mut heap = std::collections::BinaryHeap::new();
        for stone in stones {
            heap.push(stone);
        }
        while let Some(y) = heap.pop() {
            if let Some(x) = heap.pop() {
                if x != y {
                    heap.push(y - x);
                }
            } else {
                return y;
            }
        }
        0
    }
}
