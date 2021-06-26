use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
    }
    let mut p = vec![true; 55_555];
    p[0] = false;
    p[1] = false;
    for i in 2..55_555 >> 1 {
        (i << 1..)
            .step_by(i)
            .take_while(|&j| j < 55_555)
            .for_each(|j| p[j] = false);
    }
    let mod1 = p
        .iter()
        .enumerate()
        .filter_map(|(i, &b)| if i % 5 == 1 && b { Some(i) } else { None })
        .collect::<Vec<_>>();
    println!(
        "{}",
        &mod1[..n]
            .iter()
            .map(usize::to_string)
            .collect::<Vec<_>>()
            .join(" ")
    );
}
