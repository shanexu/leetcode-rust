fn main() {
    assert_eq!("Z".to_string(), Solution::convert_to_title(26));
    assert_eq!("ZY".to_string(), Solution::convert_to_title(701));
}

struct Solution;

impl Solution {
    pub fn convert_to_title(mut column_number: i32) -> String {
        let mut bs = vec![];
        while column_number > 0 {
            let mut r = column_number % 26;
            if r == 0 {
                r = 26
            }
            bs.push(b'A' + r as u8 - 1);
            column_number = (column_number - r ) / 26;
        }
        if bs.len() == 0 {
            bs.push(b'A');
        } else {
            bs.reverse();
        }
        String::from_utf8(bs).unwrap()
    }
}
