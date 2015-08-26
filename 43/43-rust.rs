
fn main() {
    let perms = permutations(1234567890.to_string()).iter().map(|x| x.parse::<u64>().unwrap() ).collect::<Vec<_>>();
    let sum = perms.iter().filter(|&x| check_special_property(*x)).fold(0, |acc, x| acc + x);
    println!("{}", sum);
}

fn check_special_property(n: u64) -> bool {
    let str = n.to_string();
    if str.len() != 10 {
        false
    } else {
        let d2 = str_to_u64(&str[1..4]);
        let d3 = str_to_u64(&str[2..5]);
        let d4 = str_to_u64(&str[3..6]);
        let d5 = str_to_u64(&str[4..7]);
        let d6 = str_to_u64(&str[5..8]);
        let d7 = str_to_u64(&str[6..9]);
        let d8 = str_to_u64(&str[7..10]);
        d2 % 2 == 0 && d3 % 3 == 0 && d4 % 5 == 0 && d5 % 7 == 0 && d6 % 11 == 0 && d7 % 13 == 0 && d8 % 17 == 0
    }
}

fn str_to_u64(s: &str) -> u64 {
    String::from(s).parse::<u64>().unwrap()
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
