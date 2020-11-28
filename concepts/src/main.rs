fn main() {

    let guess: u32 = "42".parse().expect("not a number");

    let x = 2.0;

    let y: f32 = 3.0;

    let t = true;

    let c = 'z';

    let uwu = '💜';

    let tup: (i32, f64, u8) = (500,5.4,11);

    let (x,y,z) = tup;



    println!("{}",uwu);

    println!("{}",y);


    // select part of tuple
    println!("{}",tup.0);

    let a = [1, 2, 3, 4, 5, 6, 7];

    let months = ["jan","feb","mar","apr","may","jun","aug","sep","oct","nov","dec"];


    let a: [i32; 5] = [1, 2, 3, 4, 5];

    println!("array {}",a[0]);
}

