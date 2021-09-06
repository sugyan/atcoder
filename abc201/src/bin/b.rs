use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        mut st: [(String, i32); n],
    }
    st.sort_by_cached_key(|&(_, t)| -t);
    println!("{}", st[1].0);
}
