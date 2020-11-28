fn main() {

    let num = 3;
    if num < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    if num % 4 == 0 {
        println!("is div");
    } else if num % 3 == 0 {
        println!("is div by 3");
    } 

    let num = if true { 5 } else { 6 };
    println!("Hello, world! {}",num);

    let mut counter = 0;
    let res = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };

    println!("the counter is {}", res);

    
    let mut num = 3;

    while num != 0 {
        println!("{}!",num);

        num -= 1;
    }
    println!("LIFTOFF!!!");


    let a = [10, 20, 30, 40, 50];

    let mut index = 0;

    while index < 5 {
        println!("the value is {}", a[index]);

        index += 1;
    }


    for elem in a.iter() {
        println!("iter {}", elem);
    }

    for num in (1..4).rev() {
        println!("{}!",num);
    }
}
