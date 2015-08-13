use std::collections::HashMap;

fn main() {
    let mut is_abundants: HashMap<u64,bool> = HashMap::new();
    let mut abundants = Vec::new();

    for n in (1..28124) {
        let mut has_abundant = false;
        for i in (1..n) {
            if is_abundant(i, &mut is_abundants) && is_abundant(n-i, &mut is_abundants) {
                has_abundant = true;
                break;
            }
        }
        if !has_abundant {
            abundants.push(n);
        }
    }
    println!("{:?}",abundants.iter().fold(0, |acc, x| acc + x));
}

fn proper_divisors(n: u64) -> Vec<u64> {
    let mut divisors = Vec::new();
    for i in (1..n) {
        if n % i == 0 {
            divisors.push(i);
        }
    }
    divisors
}

fn is_abundant(n: u64, cache: &mut HashMap<u64,bool>) -> bool {
    *cache.entry(n).or_insert_with(|| {
        let divisors = proper_divisors(n);
        let sum = divisors.iter().fold(0, |acc, x| x + acc);
        let abundant = sum > n;
        abundant
    })
}
