
fn main() {
    let mut limit = 100000;
    let mut primes = sieve(limit);
    let mut count = primes.len();
    while count <= 10001 {
        limit = limit * 2;
        primes = sieve(limit);
        count = primes.len();
    }
    let prime = primes.get(10000).unwrap();
    println!("{:?}", prime);
}

fn sieve(bound: u64) -> Vec<u64> {
    // start from 2
    let mut is_prime = vec![true; bound as usize + 1];
    is_prime[0] = false; is_prime[1] = false;
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
