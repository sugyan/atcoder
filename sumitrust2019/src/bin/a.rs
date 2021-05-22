use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        md: [(u8, u8); 2],
    }
    println!("{}", if md[0].0 != md[1].0 { 1 } else { 0 });
}
