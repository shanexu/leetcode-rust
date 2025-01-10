fn main() {
    let num_array = NumArray::new(vec![-2, 0, 3, -5, 2, -1]);
    println!("{}", num_array.sum_range(0, 2));
    println!("{}", num_array.sum_range(2, 5));
    println!("{}", num_array.sum_range(0, 5));
}

struct NumArray {
    prefix_sums: Vec<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl NumArray {
    fn new(nums: Vec<i32>) -> Self {
        let mut prefix_sums: Vec<i32> = vec![0; nums.len() + 1];
        let mut sum = 0;
        for i in 0..nums.len() {
            sum += nums[i];
            prefix_sums[i + 1] = sum;
        }
        NumArray { prefix_sums }
    }

    fn sum_range(&self, left: i32, right: i32) -> i32 {
        self.prefix_sums[(right + 1) as usize] - self.prefix_sums[left as usize]
    }
}
