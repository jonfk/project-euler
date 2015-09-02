
fn main() {
    let mut limit = 10000;
    let mut primes = sieve(limit);
    let mut num_consecutive = 0;
    for n in 646.. {
        if n > limit {
            limit = limit * 10;
            primes = sieve(limit);
        }
        if num_prime_factors(n, primes.clone()) == 4 {
            num_consecutive += 1;
            if num_consecutive == 4 {
                println!("{}", n);
                break;
            }
        } else {
            num_consecutive = 0;
        }
    }
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

fn prime_factors(n: u64, primes: Vec<u64>) -> Vec<u64> {
    let factors = primes.iter().filter(|&prime| n % prime == 0).map(|&x| x).collect::<Vec<_>>();
    //println!("{:?}", factors);
    factors
}

fn num_prime_factors(n: u64, primes: Vec<u64>) -> u64 {
    prime_factors(n, primes).len() as u64
}
