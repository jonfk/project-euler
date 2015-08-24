
fn main() {
    let mut max = 0;
    for n in 1..987654321 {
        let mut cur = n;
        for i in 2..10 {
            let mut temp = cur.to_string();
            temp.push_str(&(n * i).to_string());
            cur = temp.parse::<u64>().unwrap();

            if is_pandigital(cur) && temp.len() == 9 {
                if cur > max {
                    println!("{}", cur);
                    max = cur;
                }
                break;
            } else if is_partial_pandigital(cur) {

            } else {
                break;
            }
        }
    }
}

fn is_pandigital(n: u64) -> bool {
    let str = n.to_string();
    let len = str.len();
    for i in 1..len+1 {
        if !str.contains(&i.to_string()) {
            return false;
        }
    }
    return true;
}

fn is_partial_pandigital(n: u64) -> bool {
    if is_pandigital(n) {
        true
    } else {
        let str = n.to_string();
        let mut seen = Vec::new();
        for c in str.chars() {
            let as_num = c.to_digit(10).unwrap();
            if seen.contains(&as_num) {
                return false;
            }
            seen.push(as_num);
        }
        true
    }
}
