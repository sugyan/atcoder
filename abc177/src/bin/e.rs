use proconio::input;

const MAX_SIZE: usize = 1_000_001;

fn gcd(a: u32, b: u32) -> u32 {
    if a == 0 {
        b
    } else {
        gcd(b % a, a)
    }
}

fn main() {
    input! {
        n: usize,
        a: [u32; n],
    };
    let mut v = vec![0; MAX_SIZE];
    for &i in &a {
        v[i as usize] += 1;
    }
    let coprime = (2..MAX_SIZE).all(|i| {
        (i..)
            .step_by(i)
            .take_while(|&i| i < MAX_SIZE)
            .map(|i| v[i])
            .sum::<i32>()
            <= 1
    });
    if coprime {
        println!("pairwise coprime");
    } else if a.iter().fold(0, |acc, &x| gcd(acc, x)) == 1 {
        println!("setwise coprime");
    } else {
        println!("not coprime");
    }
}
