
fn main() {
    let num = 2000000;
    let primes = sieve(num);
    let sum = primes.iter().fold(0, |acc, x| acc + x);
    println!("{:?}", sum);
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

fn prime_factors(n: u64) -> Vec<u64> {
    let primes = sieve((n as f64).sqrt() as u64 + 1 );

    primes.iter().filter(|&prime| n % prime == 0).map(|&x| x).collect::<Vec<_>>()
}
