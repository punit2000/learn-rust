use unicode_segmentation::UnicodeSegmentation;
use std::collections::HashMap;

#[derive(Debug)]
enum SpreadSheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

fn main() {
    // let mut vec: Vec<i32> = Vec::new();
    let mut v1: Vec<i32> = vec![1,2,3,5,6,7,8];

    // let fourth_value = &v1[3];
    // vec.push(7);
    // vec.push(8);
    // vec.push(9);
    // let fourth_value = v1.get(30).unwrap_or(&-1);
    let fourth_value = match v1.get(30) {
        Some(value) => value,
        None => {
            println!("index out of range");
            &-1
        }
    };
    
    for i in &mut v1 {
        println!("i is {i}");
        *i = *i * 2; //dereferencing the value 
    }
    // println!("Vec = {:?} and fourth value is {}", v1);
    println!("Vec = {:?}", v1);

    let mut cells: Vec<SpreadSheetCell> = vec![
        SpreadSheetCell::Int(20),
        SpreadSheetCell::Float(2.5), 
        SpreadSheetCell::Text(String::from("hello"))
        ];
    cells.push(SpreadSheetCell::Text(String::from("New world")));
    println!("Cells = {:#?}", cells);

    match cells.get(2){
        Some(SpreadSheetCell::Int(value)) => println!("The value is int {value}"),
        Some(value) => println!("This is some other value = {:?}",value),
        None => println!("None")
    }


    let mut hello = String::from("नमस्ते");

    hello.push_str(" in hindi");
    hello.push('.');
    println!("{hello}");

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = s1 + "-" + &s2 + "-" + &s3; //ownership of s1 will be taken away
    println!("{s}");
    let s1 = String::from("tic");
    let s = format!("{s1}-{s2}-{s3}"); //wont take any ownership

    println!("{s}");

    //let first_char = &hello[0]; not possible

    for c in hello.as_bytes(){
        println!("c = {c}")
    }

    for c in hello.chars(){
        println!("c = {c}")
    }

    for e in hello.graphemes(true).collect::<Vec<&str>>(){
        println!("e: {e}");
    };
    
// [224, 164, 168, 224, 164, 174, 224, 164, 184, 224, 165, 141, 224, 164, 164, 224, 165, 135] --> bytes
// ['न', 'म', 'स', '्', 'त', 'े'] --> scalar value

    let mut scores = HashMap::new();
    let blue_team = String::from("blue"); //ownership will be taken away
    scores.insert(blue_team, 10);
    scores.insert(String::from("blue"), 100); //overwrite the value
    scores.insert(String::from("yellow"), 15);


    let score = scores.get(&String::from("blue")).unwrap_or(&0);
    //or 
    let score = scores.get(&String::from("blue")).copied().unwrap_or(0);

    println!("score: {score}");
    println!("Hashmap = {:?}",scores);

    for (key,value) in &scores {
        println!("{:?}  => {:?}", key,value)
    }

    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{map:?}");

}
