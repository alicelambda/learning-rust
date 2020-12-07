fn main() {
    // from allocates memory
    let mut s = String::from("Hello");

    s.push_str(", world!");
    let s2 = s;
    let s2 = take_own_and_give(s2);
    let mut s3 = s2.clone();
    println!("{}", s3);


    let mut pog = String::from("hello");
    change(&mut pog);
    println!("{}", calculate_length(&s3));
}

fn take_own_and_give(some: String) -> String {
    println!("{}", some);
    some
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn change(some: &mut String) {
    some.push_str(", world");
}


fn first_word(s: &str) ->  &str{
    let bytes = s.as_bytes();

    //creates iterator
    //enumerate returns a tuple
    //
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

fn slice() {
    let s = String::from("hello world");

    let hello = &s[0..5];
    let world = &s[6..11];

    let fromfrount = &s[..2];

    let toend = &s[4..];

}
