use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        s: String,
        t: String,
    }
    let yes = (t.clone() + &t).contains(&s);
    println!("{}", if yes { "Yes" } else { "No" });
}
