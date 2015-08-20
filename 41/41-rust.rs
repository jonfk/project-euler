
fn main() {
    let primes = sieve(987654321);
    let a = permutations(String::from("123456789")).iter().map(|x| x.parse::<u64>().unwrap() ).collect::<Vec<_>>();
    let l = a.iter().filter(|x| primes.contains(x)).max();
    println!("{:?}",l);

}

fn permutations(s: String) -> Vec<String> {
    if s.len() == 0 {
        vec![]
    } else if s.len() == 1 {
        vec![s]
    } else {
        let mut iter = s.chars();
        let c = iter.next().unwrap();

        let rest = iter.collect::<String>();
        let rest_permutations = permutations(rest);
        let mut all_permutations : Vec<String> = Vec::new();
        for perm in rest_permutations {
            let len = perm.len();
            for i in 0..len+1 {
                let mut new_perm = perm.clone();
                new_perm.insert(i, c);
                all_permutations.push(new_perm);
            }
        }
        all_permutations
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
