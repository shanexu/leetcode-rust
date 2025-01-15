pub fn bit_count(mut var0: u32) -> u32 {
    var0 -= var0 >> 1 & 1431655765;
    var0 = (var0 & 858993459) + (var0 >> 2 & 858993459);
    var0 = (var0 + (var0 >> 4)) & 252645135;
    var0 += var0 >> 8;
    var0 += var0 >> 16;
    var0 & 63
}

pub fn gcd(mut a: i32, mut b: i32) -> i32 {
    while b != 0 {
        let temp = b;
        b = a % b;
        a = temp;
    }
    a.abs()
}
