use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize, x: u32,
        mut a: [u32; n],
    }
    let answer = a
        .iter()
        .filter_map(|&a| if a != x { Some(a.to_string()) } else { None })
        .collect::<Vec<_>>();
    println!("{}", answer.join(" "));
}
