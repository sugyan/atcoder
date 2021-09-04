use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        _n: usize, mut x: u32,
        s: String,
    }
    for c in s.chars() {
        match c {
            'o' => x += 1,
            'x' if x > 0 => x -= 1,
            _ => {}
        }
    }
    println!("{}", x);
}
