
fn main() {
    let truncables = find_truncable_primes().iter().fold(0, |acc, x| acc + x);
    println!("{:?}", truncables);
}

fn find_truncable_primes() -> Vec<u64> {
    let mut truncable = Vec::new();
    let mut i = 11;
    let mut limit: u64 = 100000;
    let mut primes = sieve(limit);
    while truncable.len() < 11 {
        let mut truncated = truncate_right(i);
        truncated.extend(truncate_left(i));
        if truncated.iter().all(|x| primes.contains(x)) {
            truncable.push(i);
            println!("{:?}", truncable);
        }
        if i > limit {
            limit = limit * 10;
            primes = sieve(limit);
            println!("increasing limit to {}", limit)
        }
        //println!("{}", i);
        i += 1;
    }
    truncable
}

fn truncate_right(n: u64) -> Vec<u64> {
    let mut truncations = Vec::new();
    let string = n.to_string();

    for i in 1..string.len()+1 {
        let mut new_string = string.clone();
        new_string.truncate(i);
        let num = new_string.parse::<u64>().unwrap();
        truncations.push(num);
    }
    truncations
}

fn truncate_left(n: u64) -> Vec<u64> {
    let mut truncations = Vec::new();
    let string = n.to_string().chars().rev().collect::<String>();

    for i in 1..string.len()+1 {
        let mut new_string = string.clone();
        new_string.truncate(i);
        let num = new_string.chars().rev().collect::<String>().parse::<u64>().unwrap();
        truncations.push(num);
    }
    truncations
}

fn sieve(bound: u64) -> Vec<u64> {
    let mut is_prime = vec![true; bound as usize + 1];
    is_prime[0] = false; is_prime[1] = false;
    // start from 2
    for i in 2..bound {
        if is_prime[i as usize] {
            let mut j = i + i;
            while j <= bound {
                is_prime[j as usize] = false;
                j += i;
            }
        }
    }
    is_prime.iter().enumerate().filter_map(|(prime, &is_prime)| if is_prime { Some(prime as u64) } else { None }).collect::<Vec<_>>()
}
