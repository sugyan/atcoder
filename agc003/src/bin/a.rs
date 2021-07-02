use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        s: String,
    }
    let mut news = [0; 4];
    s.chars().for_each(|c| {
        news[match c {
            'N' => 0,
            'E' => 1,
            'W' => 2,
            'S' => 3,
            _ => unreachable!(),
        }] += 1
    });
    let ok = !(news[0] + news[3] > 0 && news[0] * news[3] == 0)
        && !(news[1] + news[2] > 0 && news[1] * news[2] == 0);
    println!("{}", if ok { "Yes" } else { "No" });
}
