fn main() {
    let mut set = MyHashSet::new();
    set.add(1);
    set.add(2);
    println!("{}", set.contains(1));
    println!("{}", set.contains(3));
}

struct MyHashSet {
    bit_map: Vec<u64>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyHashSet {
    fn new() -> Self {
        Self {
            bit_map: new_bit_map(1000001),
        }
    }

    fn add(&mut self, key: i32) {
        set_bit(key as usize, &mut self.bit_map);
    }

    fn remove(&mut self, key: i32) {
        unset_bit(key as usize, &mut self.bit_map);
    }

    fn contains(&mut self, key: i32) -> bool {
        get_bit(key as usize, &mut self.bit_map)
    }
}

#[inline]
fn new_bit_map(total: usize) -> Vec<u64> {
    let len = (total + 63) >> 6;
    let mut bits = vec![0; len];
    let remain = total & 63;
    if remain != 0 {
        bits[len - 1] = !0 << remain;
    }
    bits
}

#[inline]
fn get_bit(n: usize, bit_map: &mut Vec<u64>) -> bool {
    let i = n >> 6;
    let k = n & 63;
    ((bit_map[i] >> k) & 1) != 0
}

#[inline]
fn set_bit(n: usize, bit_map: &mut Vec<u64>) {
    let i = n >> 6;
    let k = n & 63;
    bit_map[i] |= 1 << k;
}

#[inline]
fn unset_bit(n: usize, bit_map: &mut Vec<u64>) {
    let i = n >> 6;
    let k = n & 63;
    bit_map[i] &= !(1 << k);
}
