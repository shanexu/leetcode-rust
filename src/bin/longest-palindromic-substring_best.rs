fn main() {
    println!("{}", Solution::longest_palindrome(String::from("aaaabbbbbbbbbbccccccccccddddddddddeeeeeeeeeeffffffffffgggggggggghhhhhhhhhhiiiiiiiiiijjjjjjjjjjkkkkkkkkkkllllllllllmmmmmmmmmmnnnnnnnnnnooooooooooppppppppppqqqqqqqqqqrrrrrrrrrrssssssssssttttttttttuuuuuuuuuuvvvvvvvvvvwwwwwwwwwwxxxxxxxxxxyyyyyyyyyyzzzzzzzzzzyyyyyyyyyyxxxxxxxxxxwwwwwwwwwwvvvvvvvvvvuuuuuuuuuuttttttttttssssssssssrrrrrrrrrrqqqqqqqqqqppppppppppoooooooooonnnnnnnnnnmmmmmmmmmmllllllllllkkkkkkkkkkjjjjjjjjjjiiiiiiiiiihhhhhhhhhhggggggggggffffffffffeeeeeeeeeeddddddddddccccccccccbbbbbbbbbbaaaaaaaabbbbbbbbbbccccccccccddddddddddeeeeeeeeeeffffffffffgggggggggghhhhhhhhhhiiiiiiiiiijjjjjjjjjjkkkkkkkkkkllllllllllmmmmmmmmmmnnnnnnnnnnooooooooooppppppppppqqqqqqqqqqrrrrrrrrrrssssssssssttttttttttuuuuuuuuuuvvvvvvvvvvwwwwwwwwwwxxxxxxxxxxyyyyyyyyyyzzzzzzzzzzyyyyyyyyyyxxxxxxxxxxwwwwwwwwwwvvvvvvvvvvuuuuuuuuuuttttttttttssssssssssrrrrrrrrrrqqqqqqqqqqppppppppppoooooooooonnnnnnnnnnmmmmmmmmmmllllllllllkkkkkkkkkkjjjjjjjjjjiiiiiiiiiihhhhhhhhhhggggggggggffffffffffeeeeeeeeeeddddddddddccccccccccbbbbbbbbbbaaaa")));
    println!("{}", Solution::longest_palindrome(String::from("babad")));

    println!("{}", Solution2::longest_palindrome(String::from("aaaabbbbbbbbbbccccccccccddddddddddeeeeeeeeeeffffffffffgggggggggghhhhhhhhhhiiiiiiiiiijjjjjjjjjjkkkkkkkkkkllllllllllmmmmmmmmmmnnnnnnnnnnooooooooooppppppppppqqqqqqqqqqrrrrrrrrrrssssssssssttttttttttuuuuuuuuuuvvvvvvvvvvwwwwwwwwwwxxxxxxxxxxyyyyyyyyyyzzzzzzzzzzyyyyyyyyyyxxxxxxxxxxwwwwwwwwwwvvvvvvvvvvuuuuuuuuuuttttttttttssssssssssrrrrrrrrrrqqqqqqqqqqppppppppppoooooooooonnnnnnnnnnmmmmmmmmmmllllllllllkkkkkkkkkkjjjjjjjjjjiiiiiiiiiihhhhhhhhhhggggggggggffffffffffeeeeeeeeeeddddddddddccccccccccbbbbbbbbbbaaaaaaaabbbbbbbbbbccccccccccddddddddddeeeeeeeeeeffffffffffgggggggggghhhhhhhhhhiiiiiiiiiijjjjjjjjjjkkkkkkkkkkllllllllllmmmmmmmmmmnnnnnnnnnnooooooooooppppppppppqqqqqqqqqqrrrrrrrrrrssssssssssttttttttttuuuuuuuuuuvvvvvvvvvvwwwwwwwwwwxxxxxxxxxxyyyyyyyyyyzzzzzzzzzzyyyyyyyyyyxxxxxxxxxxwwwwwwwwwwvvvvvvvvvvuuuuuuuuuuttttttttttssssssssssrrrrrrrrrrqqqqqqqqqqppppppppppoooooooooonnnnnnnnnnmmmmmmmmmmllllllllllkkkkkkkkkkjjjjjjjjjjiiiiiiiiiihhhhhhhhhhggggggggggffffffffffeeeeeeeeeeddddddddddccccccccccbbbbbbbbbbaaaa")));
    println!("{}", Solution2::longest_palindrome(String::from("babad")));

    println!("{}", Solution3::longest_palindrome(String::from("aaaabbbbbbbbbbccccccccccddddddddddeeeeeeeeeeffffffffffgggggggggghhhhhhhhhhiiiiiiiiiijjjjjjjjjjkkkkkkkkkkllllllllllmmmmmmmmmmnnnnnnnnnnooooooooooppppppppppqqqqqqqqqqrrrrrrrrrrssssssssssttttttttttuuuuuuuuuuvvvvvvvvvvwwwwwwwwwwxxxxxxxxxxyyyyyyyyyyzzzzzzzzzzyyyyyyyyyyxxxxxxxxxxwwwwwwwwwwvvvvvvvvvvuuuuuuuuuuttttttttttssssssssssrrrrrrrrrrqqqqqqqqqqppppppppppoooooooooonnnnnnnnnnmmmmmmmmmmllllllllllkkkkkkkkkkjjjjjjjjjjiiiiiiiiiihhhhhhhhhhggggggggggffffffffffeeeeeeeeeeddddddddddccccccccccbbbbbbbbbbaaaaaaaabbbbbbbbbbccccccccccddddddddddeeeeeeeeeeffffffffffgggggggggghhhhhhhhhhiiiiiiiiiijjjjjjjjjjkkkkkkkkkkllllllllllmmmmmmmmmmnnnnnnnnnnooooooooooppppppppppqqqqqqqqqqrrrrrrrrrrssssssssssttttttttttuuuuuuuuuuvvvvvvvvvvwwwwwwwwwwxxxxxxxxxxyyyyyyyyyyzzzzzzzzzzyyyyyyyyyyxxxxxxxxxxwwwwwwwwwwvvvvvvvvvvuuuuuuuuuuttttttttttssssssssssrrrrrrrrrrqqqqqqqqqqppppppppppoooooooooonnnnnnnnnnmmmmmmmmmmllllllllllkkkkkkkkkkjjjjjjjjjjiiiiiiiiiihhhhhhhhhhggggggggggffffffffffeeeeeeeeeeddddddddddccccccccccbbbbbbbbbbaaaa")));
    println!("{}", Solution3::longest_palindrome(String::from("babad")));

    println!("{}", Solution4::longest_palindrome(String::from("aaaabbbbbbbbbbccccccccccddddddddddeeeeeeeeeeffffffffffgggggggggghhhhhhhhhhiiiiiiiiiijjjjjjjjjjkkkkkkkkkkllllllllllmmmmmmmmmmnnnnnnnnnnooooooooooppppppppppqqqqqqqqqqrrrrrrrrrrssssssssssttttttttttuuuuuuuuuuvvvvvvvvvvwwwwwwwwwwxxxxxxxxxxyyyyyyyyyyzzzzzzzzzzyyyyyyyyyyxxxxxxxxxxwwwwwwwwwwvvvvvvvvvvuuuuuuuuuuttttttttttssssssssssrrrrrrrrrrqqqqqqqqqqppppppppppoooooooooonnnnnnnnnnmmmmmmmmmmllllllllllkkkkkkkkkkjjjjjjjjjjiiiiiiiiiihhhhhhhhhhggggggggggffffffffffeeeeeeeeeeddddddddddccccccccccbbbbbbbbbbaaaaaaaabbbbbbbbbbccccccccccddddddddddeeeeeeeeeeffffffffffgggggggggghhhhhhhhhhiiiiiiiiiijjjjjjjjjjkkkkkkkkkkllllllllllmmmmmmmmmmnnnnnnnnnnooooooooooppppppppppqqqqqqqqqqrrrrrrrrrrssssssssssttttttttttuuuuuuuuuuvvvvvvvvvvwwwwwwwwwwxxxxxxxxxxyyyyyyyyyyzzzzzzzzzzyyyyyyyyyyxxxxxxxxxxwwwwwwwwwwvvvvvvvvvvuuuuuuuuuuttttttttttssssssssssrrrrrrrrrrqqqqqqqqqqppppppppppoooooooooonnnnnnnnnnmmmmmmmmmmllllllllllkkkkkkkkkkjjjjjjjjjjiiiiiiiiiihhhhhhhhhhggggggggggffffffffffeeeeeeeeeeddddddddddccccccccccbbbbbbbbbbaaaa")));
    println!("{}", Solution4::longest_palindrome(String::from("babad")));
}

struct Solution {}

impl Solution {
    pub fn longest_palindrome(s: String) -> String {
        let bytes = s.as_bytes();
        let len = bytes.len();
        let mut mems = vec![false; len * len];
        for i in 0..len {
            mems[i * len + i] = true;
        }
        let mut m: usize = 1; // max length
        let mut mi = 0;
        let mut mj = 0;
        for i in 0..len - 1 {
            let j = i + 1;
            let e = bytes[i] == bytes[j];
            if e && 2 >= m {
                m = 2;
                mi = i;
                mj = j;
            }
            mems[i * len + j] = e;
        }
        for d in 2..len {
            for i in 0..len - d {
                let j = i + d;
                let e = bytes[i] == bytes[j] && mems[(i + 1) * len + (j - 1)];
                if e && d >= m {
                    m = d;
                    mi = i;
                    mj = j;
                }
                mems[i * len + j] = e;
            }
        }
        String::from(&s[mi..mj + 1])
    }
}

struct Solution2 {}

impl Solution2 {
    pub fn longest_palindrome(s: String) -> String {
        let bytes = s.as_bytes();
        let len = bytes.len();
        let mut mems: Vec<u8> = vec![0; (len * len + 7) >> 3];
        for i in 0..len {
            let idx = i * len + i;
            let byte_idx = idx >> 3;
            let byte_pos = idx & 7;
            mems[byte_idx] |= 1 << byte_pos;
        }
        let mut m: usize = 1; // max length
        let mut mi = 0;
        let mut mj = 0;
        for i in 0..len - 1 {
            let j = i + 1;
            let e = bytes[i] == bytes[j];
            if e && 2 >= m {
                m = 2;
                mi = i;
                mj = j;
            }
            if e {
                let idx = i * len + j;
                let byte_idx = idx >> 3;
                let byte_pos = idx & 7;
                mems[byte_idx] |= 1 << byte_pos;
            }
        }
        for d in 2..len {
            for i in 0..len - d {
                let j = i + d;
                let idx = (i + 1) * len + (j - 1);
                let byte_idx = idx >> 3;
                let byte_pos = idx & 7;
                let e = bytes[i] == bytes[j] && (mems[byte_idx] & (1 << byte_pos)) > 0;
                if e && d >= m {
                    m = d;
                    mi = i;
                    mj = j;
                }
                if e {
                    let idx = i * len + j;
                    let byte_idx = idx >> 3;
                    let byte_pos = idx & 7;
                    mems[byte_idx] |= 1 << byte_pos;
                }
            }
        }
        String::from(&s[mi..mj + 1])
    }
}

struct Solution3 {}

impl Solution3 {
    pub fn longest_palindrome(s: String) -> String {
        let bytes = s.as_bytes();
        let len = bytes.len();
        let mut mems: Vec<u64> = vec![0; (len * len + 63) >> 6];
        for i in 0..len {
            let idx = i * len + i;
            let byte_idx = idx >> 6;
            let byte_pos = idx & 63;
            mems[byte_idx] |= 1 << byte_pos;
        }
        let mut m: usize = 1; // max length
        let mut mi = 0;
        let mut mj = 0;
        for i in 0..len - 1 {
            let j = i + 1;
            let e = bytes[i] == bytes[j];
            if e && 2 >= m {
                m = 2;
                mi = i;
                mj = j;
            }
            if e {
                let idx = i * len + j;
                let byte_idx = idx >> 6;
                let byte_pos = idx & 63;
                mems[byte_idx] |= 1 << byte_pos;
            }
        }
        for d in 2..len {
            for i in 0..len - d {
                let j = i + d;
                let idx = (i + 1) * len + (j - 1);
                let byte_idx = idx >> 6;
                let byte_pos = idx & 63;
                let e = bytes[i] == bytes[j] && (mems[byte_idx] & (1 << byte_pos)) > 0;
                if e && d >= m {
                    m = d;
                    mi = i;
                    mj = j;
                }
                if e {
                    let idx = i * len + j;
                    let byte_idx = idx >> 6;
                    let byte_pos = idx & 63;
                    mems[byte_idx] |= 1 << byte_pos;
                }
            }
        }
        String::from(&s[mi..mj + 1])
    }
}

struct Solution4 {}

impl Solution4 {
    pub fn longest_palindrome(s: String) -> String {
        let bytes = s.as_bytes();
        let len = bytes.len();
        let mut mems: Vec<u64> = vec![0; ((len + 1) * len / 2 + 63) >> 6];
        for i in 0..len {
            let idx = i * len + i - i * (i + 1) / 2;
            let byte_idx = idx >> 6;
            let byte_pos = idx & 63;
            mems[byte_idx] |= 1 << byte_pos;
        }
        let mut m: usize = 1; // max length
        let mut mi = 0;
        let mut mj = 0;
        for i in 0..len - 1 {
            let j = i + 1;
            let e = bytes[i] == bytes[j];
            if e && 2 >= m {
                m = 2;
                mi = i;
                mj = j;
            }
            if e {
                let idx = i * len + j - i * (i + 1) / 2;
                let byte_idx = idx >> 6;
                let byte_pos = idx & 63;
                mems[byte_idx] |= 1 << byte_pos;
            }
        }
        for d in 2..len {
            for i in 0..len - d {
                let j = i + d;
                let idx = (i + 1) * len + (j - 1) - (i + 1) * (i + 1 + 1) / 2;
                let byte_idx = idx >> 6;
                let byte_pos = idx & 63;
                let e = bytes[i] == bytes[j] && (mems[byte_idx] & (1 << byte_pos)) > 0;
                if e && d >= m {
                    m = d;
                    mi = i;
                    mj = j;
                }
                if e {
                    let idx = i * len + j - i * (i + 1) / 2;
                    let byte_idx = idx >> 6;
                    let byte_pos = idx & 63;
                    mems[byte_idx] |= 1 << byte_pos;
                }
            }
        }
        String::from(&s[mi..mj + 1])
    }
}
