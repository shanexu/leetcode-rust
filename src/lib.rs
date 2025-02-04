pub mod list_node;
pub mod tree_node;
pub mod utils;
pub mod snippet;

#[macro_export]
macro_rules! vec_string {
    ($($x:expr),*) => {{
        vec![$(String::from($x)),*]
    }};
}

#[macro_export]
macro_rules! vec_char {
    ($($x:expr),*) => {{
        vec![$($x.chars().next().unwrap()),*]
    }};
}

#[macro_export]
macro_rules! vec_vec_i32 {
    ( $( [$($elem:expr),*] ),* ) => {{
        vec![
            $(
                vec![$($elem),*],
            )*
        ]
    }};
}

#[macro_export]
macro_rules! vec_vec_char {
    // 这是宏的主要匹配规则
    ( $($inner:expr),* ) => {{
        // 使用 vectors 来存储结果
        let mut outer_vec = Vec::new();

        // 迭代每个内层数组
        $(
            let inner_vec: Vec<char> = $inner.iter().map(|s| s.chars().next().unwrap()).collect();
            outer_vec.push(inner_vec);
        )*

            outer_vec // 返回构建的 Vec<Vec<char>>
    }};
}
