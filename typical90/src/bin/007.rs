use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        mut a: [i32; n],
        q: usize,
        b: [i32; q],
    }
    a.sort_unstable();
    for b in &b {
        let answer = match a.binary_search(b) {
            Ok(_) => 0,
            Err(i) => {
                let mut ret = if i == a.len() { b - a[i - 1] } else { a[i] - b };
                if i > 0 {
                    ret = ret.min(b - a[i - 1]);
                }
                ret
            }
        };
        println!("{}", answer);
    }
}
