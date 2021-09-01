fn main() {
    println!("{}", Solution::longest_palindrome(String::from("aaaabbbbbbbbbbccccccccccddddddddddeeeeeeeeeeffffffffffgggggggggghhhhhhhhhhiiiiiiiiiijjjjjjjjjjkkkkkkkkkkllllllllllmmmmmmmmmmnnnnnnnnnnooooooooooppppppppppqqqqqqqqqqrrrrrrrrrrssssssssssttttttttttuuuuuuuuuuvvvvvvvvvvwwwwwwwwwwxxxxxxxxxxyyyyyyyyyyzzzzzzzzzzyyyyyyyyyyxxxxxxxxxxwwwwwwwwwwvvvvvvvvvvuuuuuuuuuuttttttttttssssssssssrrrrrrrrrrqqqqqqqqqqppppppppppoooooooooonnnnnnnnnnmmmmmmmmmmllllllllllkkkkkkkkkkjjjjjjjjjjiiiiiiiiiihhhhhhhhhhggggggggggffffffffffeeeeeeeeeeddddddddddccccccccccbbbbbbbbbbaaaaaaaabbbbbbbbbbccccccccccddddddddddeeeeeeeeeeffffffffffgggggggggghhhhhhhhhhiiiiiiiiiijjjjjjjjjjkkkkkkkkkkllllllllllmmmmmmmmmmnnnnnnnnnnooooooooooppppppppppqqqqqqqqqqrrrrrrrrrrssssssssssttttttttttuuuuuuuuuuvvvvvvvvvvwwwwwwwwwwxxxxxxxxxxyyyyyyyyyyzzzzzzzzzzyyyyyyyyyyxxxxxxxxxxwwwwwwwwwwvvvvvvvvvvuuuuuuuuuuttttttttttssssssssssrrrrrrrrrrqqqqqqqqqqppppppppppoooooooooonnnnnnnnnnmmmmmmmmmmllllllllllkkkkkkkkkkjjjjjjjjjjiiiiiiiiiihhhhhhhhhhggggggggggffffffffffeeeeeeeeeeddddddddddccccccccccbbbbbbbbbbaaaa")));
    println!("{}", Solution::longest_palindrome(String::from("babad")))
}

struct Solution {}

impl Solution {
    pub fn longest_palindrome(s: String) -> String {
        let bytes = s.as_bytes();
        let len = bytes.len();
        let mut mems = vec![false; (len + 1) * len / 2];
        for i in 0..len {
            mems[i * len + i - i * (i + 1) / 2] = true;
        }
        let mut m: usize = 1;
        let mut ii = 0;
        let mut jj = 0;
        for i in 0..len - 1 {
            let j = i + 1;
            let e = bytes[i] == bytes[j];
            if e && 2 >= m {
                m = 2;
                ii = i;
                jj = j;
            }
            mems[i * len + j - i * (i + 1) / 2] = e;
        }
        for d in 2..len {
            for i in 0..len - d {
                let j = i + d;
                let e = bytes[i] == bytes[j]
                    && mems[(i + 1) * len + (j - 1) - (i + 1) * (i + 1 + 1) / 2];
                if e && d >= m {
                    m = d;
                    ii = i;
                    jj = j;
                }
                mems[i * len + j - i * (i + 1) / 2] = e;
            }
        }
        String::from(&s[ii..jj + 1])
    }
}
