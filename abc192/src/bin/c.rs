use proconio::{fastout, input};

fn f(n: u32) -> u32 {
    let mut s = n.to_string().chars().collect::<Vec<_>>();
    s.sort_unstable();
    s.iter().rev().collect::<String>().parse::<u32>().unwrap()
        - s.iter().collect::<String>().parse::<u32>().unwrap()
}

#[fastout]
fn main() {
    input! {
        mut n: u32, k: usize,
    }
    for _ in 0..k {
        n = f(n);
    }
    println!("{}", n);
}
