fn main() {
    println!(
        "{:?}",
        Solution::sort_by_bits(vec![0, 1, 2, 3, 4, 5, 6, 7, 8])
    );
    println!(
        "{:?}",
        Solution::sort_by_bits(vec![1024, 512, 256, 128, 64, 32, 16, 8, 4, 2, 1])
    );
}

struct Solution;

impl Solution {
    pub fn sort_by_bits(arr: Vec<i32>) -> Vec<i32> {
        let mut arr = arr;
        arr.sort_by(|a, b| bit_count(*a).cmp(&bit_count(*b)).then(a.cmp(b)));
        arr
    }
}

pub fn bit_count(mut var0: i32) -> i32 {
    var0 -= var0 >> 1 & 1431655765;
    var0 = (var0 & 858993459) + (var0 >> 2 & 858993459);
    var0 = (var0 + (var0 >> 4)) & 252645135;
    var0 += var0 >> 8;
    var0 += var0 >> 16;
    var0 & 63
}
