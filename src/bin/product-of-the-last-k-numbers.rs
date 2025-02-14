fn main() {
    let mut pon = ProductOfNumbers::new();
    pon.add(3);
    pon.add(0);
    pon.add(2);
    pon.add(5);
    pon.add(4);
    assert_eq!(20, pon.get_product(2));
    assert_eq!(40, pon.get_product(3));
    assert_eq!(0, pon.get_product(4));
    pon.add(8);
    assert_eq!(32, pon.get_product(2));
}

struct ProductOfNumbers {
    nums: Vec<u64>,
}

impl ProductOfNumbers {
    fn new() -> Self {
        Self { nums: vec![] }
    }

    fn add(&mut self, num: i32) {
        if num == 0 {
            self.nums.clear();
        } else {
            let num = num as u64;
            match self.nums.last() {
                Some(&last) => self.nums.push(last * num),
                None => self.nums.push(num),
            }
        }
    }

    fn get_product(&self, k: i32) -> i32 {
        let n = self.nums.len();
        let k = k as usize;
        let (i, overflowing) = n.overflowing_sub(k);
        if overflowing {
            0
        } else if i == 0 {
            self.nums[n - 1] as _
        } else {
            (self.nums[n - 1] / self.nums[i - 1]) as _
        }
    }
}
