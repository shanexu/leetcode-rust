fn main() {
    let mut rle = RLEIterator::new(vec![3, 8, 0, 9, 2, 5]);
    println!("{}", rle.next(2));
    println!("{}", rle.next(1));
    println!("{}", rle.next(1));
    println!("{}", rle.next(2));
}

struct RLEIterator {
    encoding: Vec<i32>,
    i: usize,
    j: usize,
}

impl RLEIterator {
    fn new(encoding: Vec<i32>) -> Self {
        Self {
            encoding,
            i: 0,
            j: 0,
        }
    }

    fn next(&mut self, n: i32) -> i32 {
        let mut n = n as usize;
        while n > 0 && self.i < self.encoding.len() {
            if n > self.encoding[self.i] as usize - self.j {
                n -= self.encoding[self.i] as usize - self.j;
                self.i += 2;
                self.j = 0;
            } else {
                self.j += n;
                return self.encoding[self.i + 1];
            }
        }
        -1
    }
}
