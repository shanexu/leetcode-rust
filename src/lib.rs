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
