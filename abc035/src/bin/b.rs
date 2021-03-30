use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        s: Chars,
        t: u8,
    }
    let mut p = (0_i32, 0_i32);
    let mut q = 0;
    for c in s {
        match c {
            'L' => p.0 -= 1,
            'R' => p.0 += 1,
            'U' => p.1 += 1,
            'D' => p.1 -= 1,
            '?' => q += 1,
            _ => unreachable!(),
        }
    }
    let answer = match t {
        1 => p.0.abs() + p.1.abs() + q,
        2 => {
            let a = p.0.abs() + p.1.abs() - q;
            a.max(a.rem_euclid(2))
        }
        _ => unreachable!(),
    };
    println!("{}", answer);
}
