use std::collections::HashMap;

fn main() {

    let v: Vec<i32> = Vec::new();
    let v = vec![1, 2, 3];
    let third: &i32 = &v[2];

    match v.get(2) {
        Some(third) => println!("jkl {}", third),
        None => println!("there is no third"),
    }

    for i in &v {
        println!("{}", i);
    }
    println!("Hello, world!");

    let mut x = Vec::new();

    x.push(5);
    x.push(5);
    x.push(7);
    x.push(8);

    println!("{:?}",v);


    for i in &mut x {
        *i += 50;
    }

    println!("{:?}",x);

    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2;


    let t1 = String::from("tic");
    let t2 = String::from("tac");
    let t3 = String::from("toe");

    let t = format!("{}-{}-{}",t1,t2,t3);

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"),10);
    scores.insert(String::from("Yellow"),50);

    
    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let init_vals = vec![10,50];

    let mut scores: HashMap<_,_> =
        teams.into_iter().zip(init_vals.into_iter()).collect();


    let field_name = String::from("favorite color");
    let field_val = String::from("blue");

    let mut map = HashMap::new();

    map.insert(field_name,field_val);

    //field_name and field_val are now unusable because map is now their owner
    
   
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"),10);
    scores.insert(String::from("Yellow"),50);

    let team_name = String::from("Blue");

    let score = scores.get(&team_name);

    for (key, value) in &scores {
        println!("{}: {}", key,value);
    }


    //overwrite value
    scores.insert(String::from("Blue"), 24);

    //insert if it has no value
    scores.entry(String::from("Purple")).or_insert(54);


    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}",map);


}
