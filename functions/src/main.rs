fn main() {
    println!("Hello, world!");

    another_function(5);
    two_func(3,4);

    uwwu();

    println!("{}",five());
}

fn another_function(x: i32) {
    println!("Another function.");
    println!("The value of x is: {}",x);
}

fn two_func(x: i32, y: i32) {
    println!("the value of x is {}", x);
    println!("the value of y is {}", y);
}

fn uwwu() {
    let x = 5;
    let y = {
        let x = 3;
        x + 1
    };

    println!("the value of y is: {}", y);
}

fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}
