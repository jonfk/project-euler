
fn main() {
    let rotations = rotations(64);
    println!("{:?}", rotations);
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
