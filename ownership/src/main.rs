fn main() {
    // from allocates memory
    let mut s = String::from("Hello");

    s.push_str(", world!");
    let s2 = s;
    println!("{}", s);
}
