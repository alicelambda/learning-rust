fn main() {
    // from allocates memory
    let mut s = String::from("Hello");

    s.push_str(", world!");
    let s2 = s;
    take_own(s2);
    let s3 = s2.clone();
    println!("{}", s3);
}

fn take_own(some: String) {
    println!("{}", some);
}
