
fn main() {
    let mut min = 100000;
    let pents = generate_pentagonal_nums(10000);

    for i in pents.clone() {
        for j in pents.clone() {
            let diff = (i - j).abs();
            if check_sum_difference_pentagonal(i,j) && diff < min {
                min = diff;
                println!("{}", min);
            }
        }
    }
    println!("{}", min);
}

fn generate_pentagonal_nums(limit: i64) -> Vec<i64> {
    let mut result = Vec::new();
    for i in 1..limit+1 {
        let pen = i * ((3 * i) - 1) / 2;
        result.push(pen);
    }
    result
}

fn check_sum_difference_pentagonal(a: i64, b: i64) -> bool {
    let sum = a + b;
    let diff = (a - b).abs();
    let max = if sum > diff {sum} else {diff};
    let limit = ((max as f64) * 3.0 / 2.0).sqrt() as i64 + 1;
    let pents = generate_pentagonal_nums(limit);
    pents.contains(&sum) && pents.contains(&diff)
}
