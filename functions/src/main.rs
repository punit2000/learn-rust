use core::num;

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