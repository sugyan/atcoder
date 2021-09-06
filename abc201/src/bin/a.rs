use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        mut a: [u8; 3],
    }
    a.sort();
    let yes = a[2] - a[1] == a[1] - a[0];
    println!("{}", if yes { "Yes" } else { "No" });
}
