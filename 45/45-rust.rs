

fn main() {
    let mut i = 286;
    loop {
        let triangular = gen_triangular(i);
        if is_pentagonal(triangular) && is_hexagonal(triangular) {
            println!("{}", triangular);
            break;
        }
        i += 1;
    }
}


/*
 * Triangular, Pentagonal and Hexagonal helpers
 */

fn is_triangular(x: i64) -> bool {
    let c = - (2 * x);
    let sol = positive_quadratic_solution(1,1,c);
    is_integer(sol)
}

fn is_pentagonal(x: i64) -> bool {
    let c = - (2 * x);
    let sol = positive_quadratic_solution(3,-1,c);
    is_integer(sol)
}

fn is_hexagonal(x: i64) -> bool {
    let sol = positive_quadratic_solution(2, -1, -x);
    is_integer(sol)
}

fn gen_triangular(x: i64) -> i64 {
    x * (x + 1) / 2
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
