use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        a: [i8; 3],
    }
    let mut v = vec![
        (a[0] - a[1]).abs(),
        (a[0] - a[2]).abs(),
        (a[1] - a[2]).abs(),
    ];
    v.sort_unstable();
    println!("{}", v[0] + v[1]);
}
