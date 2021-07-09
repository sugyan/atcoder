use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        mut ac: [(i64, char); n * 2],
    }
    let mut rgb = vec![Vec::new(); 3];
    for &(a, c) in &ac {
        let i = match c {
            'R' => 0,
            'G' => 1,
            'B' => 2,
            _ => unreachable!(),
        };
        rgb[i].push(a);
    }
    if rgb.iter().all(|v| v.len() & 1 == 0) {
        println!("0");
        return;
    }
    rgb.iter_mut().for_each(|v| v.sort_unstable());
    let m = |i: usize, j: usize| -> i64 {
        if rgb[i].is_empty() {
            return rgb[j][0];
        }
        if rgb[j].is_empty() {
            return rgb[i][0];
        }
        rgb[i].iter().fold(std::i64::MAX, |acc, x| {
            acc.min(match rgb[j].binary_search(x) {
                Ok(_) => 0,
                Err(k) => (rgb[j][k.max(1) - 1] - x)
                    .abs()
                    .min((rgb[j][k.min(rgb[j].len() - 1)] - x).abs()),
            })
        })
    };
    if let Some(e) = rgb.iter().position(|v| v.len() & 1 == 0) {
        let answer = m((e + 1) % 3, (e + 2) % 3).min(m(e, (e + 1) % 3) + m(e, (e + 2) % 3));
        println!("{}", answer);
    }
}
