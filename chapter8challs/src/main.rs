


fn main() {
    let mut v: Vec<i32> = Vec::new();
    v.push(1);
    v.push(2);
    v.push(3);
    v.push(4);


    println!("{}",mean(v));
}

fn mean(v :Vec<i32>) -> f64 {

    let mut sum = 0.0;
    for i in v.iter() {
        sum += f64::from(*i);
    }

    sum/ v.len() as f64

}

