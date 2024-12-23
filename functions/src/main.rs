use core::num;
use std::ops::Index;

fn main() {
    let num = 100;
    my_function(num, true); //fn calling (it is an expression)
    let y = {
        let x = 3;
        x + 1;
        10
    }; // if ";" then it is a statement ..if not then expression and it will return a value
    println!("y: {y}");
    let y = add(5, 10);
    println!("Value of y is {y}");
    let z = is_even(11);
    println!("The value of z is {z}");

    //if y is true, x = 10, else x = 20
    let y = is_even(10);
    let x = if y {10} else {20};
    println!("x: {x}");
    // three types of loops: loop, while and for
    let mut num =  1;
    let result = 'my_loop: loop {
         println!("value of number is {num}");
        //  if num == 5 {
        //     continue;
        //  }
         if num == 10 {
            break 80; 
         }
         loop {             
             if num == 20 {
                break 'my_loop 40; //will work for outer loop as well
             }
             num = num + 1;
         }
         num = num + 1;
    };

    let mut num = 3;
    //while loop
    while num != 0 {
        println!("{num}!");
        num -= 1;
    }

    let arr = [1,2,3,4,5,6];
    let mut index = 0;
    // not an efficient way !!
    while index < 6 {
        println!("i: {} and v: {}", index, arr[index]);
        index += 1;
    }
    println!("result: {result}");

    //for loop
    for x in arr {
        println!("x: {x}");
    }

    for x in (1..=10).rev() {
        println!("x is {x}");
    }
}

fn my_function(x: i32, y: bool) {
    println!("Hello from my function {x} {y}");
}

fn add(x: i32, y: i32) -> i32 {
    x + y
}

fn is_even(x: i32) -> bool {
    if  x%2 ==0 {
        return true; //for early return
    }
    println!("The number is not even");
    false
}