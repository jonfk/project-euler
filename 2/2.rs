
fn main() {
    let mut numbers = Vec::<i32>::new();
    let mut i = 1;
    let mut last = 1;

    while last < 4000000 {
        last = fib(i);
        numbers.push(last);
        i += 1;
    }
    let result = numbers.iter().filter(|x| *x % 2 == 0).fold(0, |acc, x| acc + x);
    println!("result : {}", result);
}

// no guaranteed tail call elimination in rust
fn fib(n: i32) -> i32 {
    let mut x = 0;
    let mut y = 1;
    for i in (1..n) {
        let temp = x + y;
        x = y;
        y = temp;
    }
    return y;
}
