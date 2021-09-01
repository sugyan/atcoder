use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        _n: usize, m: usize,
        mut ab: [(usize, usize); m],
    }
    ab.sort();
    let mut v = Vec::new();
    for &(a, b) in &ab {
        if let Some(min) = v.last_mut().filter(|min| a < **min) {
            *min = b.min(*min);
        } else {
            v.push(b);
        }
    }
    println!("{}", v.len());
}
