fn main() {
    println!("{:?}", Solution::letter_combinations(String::from("22")));
    // let mut vs = vec![String::with_capacity(3)];
    // for s in vs.iter_mut() {
    //     s.insert(0, 'b');
    //     println!("{}", s);
    // }
    // println!("{:?}", vs);
}

struct Solution {}

impl Solution {
    pub fn letter_combinations(digits: String) -> Vec<String> {
        let dl = digits.len();
        if dl == 0 {
            return vec![];
        }
        let pad = [
            vec![b'a', b'b', b'c'],
            vec![b'd', b'e', b'f'],
            vec![b'g', b'h', b'i'],
            vec![b'j', b'k', b'l'],
            vec![b'm', b'n', b'o'],
            vec![b'p', b'q', b'r', b's'],
            vec![b't', b'u', b'v'],
            vec![b'w', b'x', b'y', b'z'],
        ];
        let digits = digits.as_bytes();
        let total = digits
            .iter()
            .map(|&b| if b == b'9' || b == b'7' { 4 } else { 3 })
            .product::<usize>();
        let mut results = Vec::with_capacity(total);
        let mut vec: Vec<usize> = vec![0; dl];
        for i in 0..total {
            let mut str_vec: Vec<u8> = Vec::with_capacity(dl);
            for (di, &vi) in vec.iter().enumerate() {
                str_vec.push(pad[(digits[di] - b'2') as usize][vi]);
            }
            results.push(String::from_utf8(str_vec).unwrap());
            if i == total - 1 {
                break;
            }
            for j in 0..dl {
                vec[j] = vec[j] + 1;
                if vec[j] == pad[(digits[j] - b'2') as usize].len() {
                    vec[j] = 0;
                } else {
                    break;
                }
            }
        }
        results
    }
}
