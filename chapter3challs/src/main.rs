fn main() {
    println!("2 {}", nfib(5));
    println!("32 {}",f2c(32.0));
}

fn nfib(x: i32) -> i32 {

    if x  < 2 {
        1
    } else {
        nfib(x-1) + nfib(x-2)
    }
}

fn f2c (x: f64) -> f64 {
    (x-32.0) * (5.0/9.0)
}
