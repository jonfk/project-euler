
fn main() {
    let primes = sieve(1000000);
    let mut circular_primes: Vec<u64> = Vec::new();
    for n in primes.clone() {
        if is_circular_prime(n, primes.clone()) {
            circular_primes.push(n);
        }
    }
    println!("{}",circular_primes.len());
}

fn rotations(n: u64) -> Vec<u64> {
    let str = n.to_string();
    if str.len() < 2 {
        vec![n]
    } else {
        let mut rotations = Vec::new();
        let mut rotation = rotate_once(str.clone());
        rotations.push(rotation.clone());
        while rotation != str {
            rotation = rotate_once(rotation.clone());
            rotations.push(rotation.clone());
        }
        rotations.iter().map(|x| x.parse::<u64>().unwrap()).collect::<Vec<_>>()
    }
}

fn rotate_once(str: String) -> String {
    let mut char_iter = str.chars();
    let first_char = char_iter.next().unwrap();
    let mut rest = char_iter.collect::<String>();
    rest.push(first_char);
    rest
}

fn is_circular_prime(n: u64, primes: Vec<u64>) -> bool {
    let rotations = rotations(n);
    //println!("{:?}", rotations);
    for i in rotations {
        if !primes.contains(&i) {
            return false;
        }
    }
    true
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
