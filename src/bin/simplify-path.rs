fn main() {
    // println!(
    //     "{}",
    //     Solution::simplify_path("//hello////world".to_string())
    // );
    // println!(
    //     "{}",
    //     Solution::simplify_path("//hello/.///world".to_string())
    // );
    println!(
        "{}",
        Solution::simplify_path("//hello/../world".to_string())
    );
    // println!(
    //     "{}",
    //     Solution::simplify_path("//hello/...///world".to_string())
    // );
    // println!(
    //     "{}",
    //     Solution::simplify_path("//hello/...///world/".to_string())
    // );
    println!("{}", Solution::simplify_path("/..".to_string()));
}

struct Solution;

impl Solution {
    pub fn simplify_path(path: String) -> String {
        let mut ans: Vec<u8> = vec![];
        let path = path.as_bytes();
        for &b in path {
            if b == b'/' {
                while !ans.is_empty() && ans[ans.len() - 1] == b'/' {
                    ans.pop();
                }
                if ans.ends_with(&[b'/', b'.']) {
                    ans.pop();
                    ans.pop();
                }
                if ans.ends_with(&[b'/', b'.', b'.']) {
                    ans.pop();
                    ans.pop();
                    ans.pop();
                    while !ans.is_empty() && ans[ans.len() - 1] != b'/' {
                        ans.pop();
                    }
                    if !ans.is_empty() && ans[ans.len() - 1] == b'/' {
                        ans.pop();
                    }
                }
                ans.push(b);
            } else {
                ans.push(b);
            }
        }
        if ans.ends_with(&[b'/', b'.']) {
            ans.pop();
            if ans.len() != 1 {
                ans.pop();
            }
        }
        if ans.ends_with(&[b'/', b'.', b'.']) {
            ans.pop();
            ans.pop();
            if ans.len() != 1 {
                ans.pop();
            }
            while !ans.is_empty() && ans[ans.len() - 1] != b'/' {
                ans.pop();
            }
        }
        if ans.len() > 1 {
            if ans[ans.len() - 1] == b'/' {
                ans.pop();
            }
        }
        String::from_utf8(ans).unwrap()
    }
}
