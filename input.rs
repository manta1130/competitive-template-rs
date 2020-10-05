use std::cell::RefCell;
use std::io;
pub const SPLIT_DELIMITER: char = ' ';
pub use std::io::prelude::*;

#[macro_export]
thread_local! {
    pub static INPUT_BUFFER:RefCell<std::collections::VecDeque<String>>=RefCell::new(std::collections::VecDeque::new());
}

/// 空白で区切られた複数の値の読み込む。
/// # Example
/// ```ignore
/// input!(a:usize,b:usize);
/// ```
#[macro_export]
#[allow(unused_macros)]
macro_rules! inputm {
    ( $($x:ident : $t:ty),*) => {
                $(
                INPUT_BUFFER.with(|p| if p.borrow().len()==0{
                    let temp_str = input_line_str();
                    let mut split_result_iter = temp_str.split(SPLIT_DELIMITER).map(|q|q.to_string()).collect::<std::collections::VecDeque<_>>();
                    p.borrow_mut().append(&mut split_result_iter)
                });
                let mut buf_split_result=String::new();
                INPUT_BUFFER.with(|p| buf_split_result = p.borrow_mut().pop_front().unwrap());
                    let ($x):($t) = buf_split_result.parse().unwrap();
                )*
    };
}

/// 空白で区切られた複数の値の読み込む。
/// # Example
/// ```ignore
/// input!(a:usize,b:usize);
/// ```
#[macro_export]
#[allow(unused_macros)]
macro_rules! input_all {
    ( $($x:ident : $t:ty),*) => {
                $(
                INPUT_BUFFER.with(|p| if p.borrow().len()==0{
                    let mut temp_str = String::new();
                    std::io::stdin().read_to_string(&mut temp_str).unwrap();
                    let mut split_result_iter = temp_str.split_whitespace().map(|q|q.to_string()).collect::<std::collections::VecDeque<_>>();
                    p.borrow_mut().append(&mut split_result_iter)
                });
                let mut buf_split_result=String::new();
                INPUT_BUFFER.with(|p| buf_split_result = p.borrow_mut().pop_front().unwrap());
                    let ($x):($t) = buf_split_result.parse().unwrap();
                )*
    };
}

/// 文字列を一行読み込む
/// # Example
/// ```ignore
/// let s = input_line_str();
/// ```
pub fn input_line_str() -> String {
    let mut s = String::new();
    io::stdin().read_line(&mut s).unwrap();
    s.trim().to_string()
}

/// 一行読み込み、配列(Vec)に変換する。
/// # Examples
/// ```ignore
/// let v=input_vector::<usize>();
/// ```
#[allow(clippy::match_wild_err_arm)]
pub fn input_vector<T>() -> Vec<T>
where
    T: std::str::FromStr,
{
    let mut v: Vec<T> = Vec::new();

    let s = input_line_str();
    let split_result = s.split(SPLIT_DELIMITER);
    for z in split_result {
        let buf = match z.parse() {
            Ok(r) => r,
            Err(_) => panic!("Parse Error",),
        };
        v.push(buf);
    }
    v
}

///　指定された行数を読み込む
#[allow(clippy::match_wild_err_arm)]
pub fn input_vector_row<T>(n: usize) -> Vec<T>
where
    T: std::str::FromStr,
{
    let mut v = Vec::with_capacity(n);
    for _ in 0..n {
        let buf = match input_line_str().parse() {
            Ok(r) => r,
            Err(_) => panic!("Parse Error",),
        };
        v.push(buf);
    }
    v
}

/// StringをVec<char>に変換するトレイト
pub trait ToCharVec {
    fn to_charvec(&self) -> Vec<char>;
}

impl ToCharVec for String {
    fn to_charvec(&self) -> Vec<char> {
        self.to_string().chars().collect::<Vec<_>>()
    }
}
