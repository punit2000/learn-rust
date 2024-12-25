use std::{result, string};

fn main() {
    let s = "Hello World"; // main function is the owner
    {
        let x = "Hey from x"; // x scope starts
    } //x scope ends

    let mut s = String::from("Hello"); //growable
    s.push_str(" World");
    //drop(s); this function is called to drop the variable out of scope.
    println!("S:  {s}");

    let mut x = 5;
    let y = x; //stack only copy
    x = 10;

    println!("X is {x} and Y is {y}");

    let x = String::from("I am X"); //goes into heap
    let mut y = x.clone(); // expensive
                           //println!("x is {x}") x got invalidated for let y = x
    y.push_str("This is y");
    println!("x is {x}");
    println!("y is {y}");

    let num = 15;
    let result = add(num);
    let name = String::from("Punit Savlesha");
    let s = gives_ownership();
    let mut s = takes_and_gives_back(s);
    takes_ownership(name);
    println!("S is {s}");
    println!("Num is {num} and result is {result}");

    let len = calculate_len(&mut s); //called as borrowing i.e. referencing 
    println!("The len of {s} is {len}");
    //println!("Value of name is  {name} ") owner is takes_ownership functions

    //slices
    let mut str = String::from("Hello World");
    let res = find_first_word(&str);

    //let hello = &str[..5];
    //let world = &str[6..];
    //str.clear();
    println!("String is {str} and Result is {}", res.len());

}

fn takes_ownership(s: String) {
    println!("Inside ownership {s}")
}

fn gives_ownership() -> String {
    let s = String::from("This is a string from the give ownership function");
    s
}

fn takes_and_gives_back(s: String) -> String {
    println!("S in takes and gives back {s}");
    s
}

fn add(x: i32) -> i32 {
    x + 10
}

fn calculate_len(s: &mut String) -> usize {
    s.push_str("World"); 
    let result = s.len(); 
    result
}

fn find_first_word(input: &str) -> &str {
    let s = input.as_bytes();
    for (i, &item) in s.iter().enumerate() {
        if item == b' ' {
            return &input[..i];
        }
    }

    &input[..]
}
