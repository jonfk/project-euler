
fn main() {

    let palindromes = (1..1000000).filter_map(|x| {
        if is_palindrome(format!("{}",x)) && is_palindrome(format!("{:b}",x)) {
            Some(x)
        } else {
            None
        }
    }).collect::<Vec<u64>>();

    let sum = palindromes.iter().fold(0, |acc,x| acc + x);
    println!("{}", sum);
}

fn is_palindrome(n: String) -> bool {
    n == n.chars().rev().collect::<String>()
}
