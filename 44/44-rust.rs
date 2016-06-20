
fn main() {
    let limit = 1000000;
    let mut min = 1000;
    let pents = generate_pentagonal_nums(limit);

    for i in pents.clone() {
        let min2 = min;
        for j in pents.clone().iter().take_while(|&x| (x - i).abs() < min2) {
            let diff = (i - *j).abs();
            if check_sum_difference_pentagonal(i,*j) && diff < min {
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
    is_pentagonal(sum) && is_pentagonal(diff)
}

fn positive_quadratic_solution(a: i64, b: i64, c: i64) -> f64 {
    let (x, x2) = solve_quadratic(a,b,c);
    if x > 0.0 { x } else { x2 }
}

fn solve_quadratic(ai: i64, bi: i64, ci: i64) -> (f64, f64) {
    // x = (-b +- sqrt(b^2 - 4(ac))) / 2a
    let (a ,b ,c) = (ai as f64, bi as f64, ci as f64);

    let temp = ((b * b) - (4.0 * a * c)).sqrt();
    let x = (-b + temp) / (2.0 * a);
    let x2 = (-b - temp) / (2.0 * a);
    (x, x2)
}

fn is_integer(x: f64) -> bool {
    x.ceil() == x && x.floor() == x
}

fn is_pentagonal(x: i64) -> bool {
    let c = - (2 * x);
    let sol = positive_quadratic_solution(3,-1,c);
    is_integer(sol)
}
