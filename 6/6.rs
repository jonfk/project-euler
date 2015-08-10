

fn main() {
    let sum_product = (1..101).map(|x| x * x).fold(0, |acc, x| acc + x);
    let product_sum = (1..101).fold(0, |acc, x| acc + x);
    let sol = (product_sum * product_sum) - sum_product;
    println!("{:?}", sol);
}
