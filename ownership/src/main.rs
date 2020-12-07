fn main() {
    // from allocates memory
    let mut s = String::from("Hello");

    s.push_str(", world!");
    let s2 = s;
    let s2 = take_own_and_give(s2);
    let s3 = s2.clone();
    println!("{}", s3);

    println!("{}", calculate_length(&s3));
}

fn take_own_and_give(some: String) -> String {
    println!("{}", some);
    some
}

fn calculate_length(s: &String) -> usize {
    s.len()
}
