//enum can only be one of its variants
enum IpAddrKind {
    V4(u8,u8,u8,u8),
    V6(String),
}



enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32,i32,i32),
}


fn main() {



    

    println!("Hello, world!");

    let some_number = Some(5);
    let some_string = Some("a string");

    //rust has to define a type of none
    let absent_number: Option<i32> = None;

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
}


enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}

fn value_in_cents(coin: Coin) -> u8 {

    //compare against patterns
    match coin {
        //pattern and code
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
    }

}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i +1),
    }
}

//under bar matches placeholdes
fn print_u8(val: u8) {
    match val {
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("three"),
        4 => println!("four"),
        _ => (),
    }
}




fn print_u8_if_3 (val: Option<u8>) {
    //match on one specific value
    if let Some(3) = val {
        println!("three");
    }
}

fn coin_func(coin :Coin) {
    //if let and else
    let mut count = 0;
    if let Coin::Quarter(state) = coin {
        println!("State quarter from {:?}!",state);
    } else {
        count += 1;
    }
}
