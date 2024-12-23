fn main() {
    let mut s = StockSpanner::new();
    println!("{:?}", s.next(100));
    println!("{:?}", s.next(80));
    println!("{:?}", s.next(60));
    println!("{:?}", s.next(70));
    println!("{:?}", s.next(60));
    println!("{:?}", s.next(75));
    println!("{:?}", s.next(85));
}

struct StockSpanner {
    stack: Vec<(i32, usize)>,
    size: usize,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl StockSpanner {
    fn new() -> Self {
        Self { stack: vec![], size: 0 }
    }

    fn next(&mut self, price: i32) -> i32 {
        while !self.stack.is_empty() && self.stack[self.stack.len() - 1].0 <= price {
            self.stack.pop();
        }
        let ans = if self.stack.is_empty() {
            self.size + 1
        } else {
            self.size - self.stack[self.stack.len() - 1].1
        };
        self.stack.push((price, self.size));
        self.size += 1;
        ans as i32
    }
}
