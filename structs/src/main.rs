struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    println!("Hello, world!");


    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    let mut usr1 = User {
        email: String::from("example@example.com"),
        username: String::from("a1c3"),
        active: true,
        sign_in_count: 1,
    };

    usr1.email = String::from("anotherone@example.com");


    let usr2 = User {
        email: String::from("g@g.f"),
        username: String::from("anotherone"),
        active: usr1.active,
        sign_in_count: usr1.sign_in_count,
    };

    let usr3 = User {
        email: String::from("f@g.com"),
        username: String::from("hewwor"),
        ..usr1
    };


    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);

    let black = Color(0,0,0);
    let origin = Point(0,0,0);

    println!(
        "The area of the rectangle is {} square pixels.",
        area(&rect1)
        );
    println!("rect1 is {:?}", rect1);
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}
