fn main() {
    let mut min_stack = MinStack::new();
    min_stack.push(-2);
    min_stack.push(0);
    min_stack.push(3);
    min_stack.pop();
    println!("{}", min_stack.top());
    println!("{}", min_stack.get_min());
}

struct MinStack {
    data: Vec<(i32, i32)>,
}

impl MinStack {
    fn new() -> Self {
        MinStack { data: vec![] }
    }

    fn push(&mut self, val: i32) {
        if self.data.len() == 0 {
            self.data.push((val, val));
        } else {
            let (_, min_val) = self.data[self.data.len() - 1];
            self.data.push((val, min_val.min(val)));
        }
    }

    fn pop(&mut self) {
        self.data.pop();
    }

    fn top(&self) -> i32 {
        self.data[self.data.len() - 1].0
    }

    fn get_min(&self) -> i32 {
        self.data[self.data.len() - 1].1
    }
}
