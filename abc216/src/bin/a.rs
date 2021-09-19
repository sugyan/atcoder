use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        xy: String,
    }
    let v = xy
        .split('.')
        .filter_map(|s| s.parse::<u8>().ok())
        .collect::<Vec<_>>();
    let s = match v[1] {
        0..=2 => "-",
        3..=6 => "",
        _ => "+",
    };
    println!("{}{}", v[0], s);
}
