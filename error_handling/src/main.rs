use std::fs::{self, File};
use std::io::{self, Read};
fn main() {
    println!("Hello, world!");
    //panic!("this is my panic");
    //println!("this is end of the code")

    let arr = [1,2,3,4,5];

    //let el = arr[10];
    //panic!("the el at 10th index is {el}");

    //let r = divide(4,0).unwrap_or(-1);
    let r = match divide(4,0)  {
        Ok(num) => num,
        Err(err) => {
            println!("Error: {err}");
            -1
        }
    }; 
    println!("R = {r}");

    let greeting_file_result = File::open("Hello.txt");
    //provide custom error using .expect() function 
    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        //Err(error) => panic!("Problem opening the file: {error:?}")
        Err(error) => match error.kind() {
            std::io::ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(file) => file,
                Err(error) => panic!("Problem creating the file: {error:?}")  
            },
            _ => panic!("Problem opening the file: {error:?}")  
        }
    };
    //let greeting_file_result = File::open("Hello.txt").expect("Failed to open the file");

    let username = read_username_from_file().expect("Failed to read the file");
    println!("Username: {}", username);

}

fn divide(x:i32,y:i32) -> Result<i32, String> {
    if y == 0 {
        return Err(String::from("Please do not divide by zero"));
    }

    Ok(x/y)
}

fn read_username_from_file() -> Result<String, io::Error> {
    fs::read_to_string("hello.txt")
}


