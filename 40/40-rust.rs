
fn main() {
    let mut str = String::new();
    for i in 1..1000001 {
        str.push_str(&i.to_string());
    }
    let mut d = Vec::new();

    let mut i = 1;
    while i < 1000001 {
        let char = str.chars().nth(i-1).unwrap();
        let num = char.to_string().parse::<u64>().unwrap();
        d.push(num);
        i = i * 10;
    }
    let product = d.iter().fold(1, |acc, x| acc * x);
    println!("{}", product);
}
