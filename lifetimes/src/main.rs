fn main() {
    // let result: &i32 ;
    // {
    //     let y = 10;
    //     result = &y;
        
    // }
    // println!("Result = {result}");
// string literals have 'static lifetime i.e they are available through the program
    let s1 = String::from("Punit");
    let result: &str;
    {
        let s2 = String::from("Punit Savlesha");
        result = longest(&s1, &s2);
    }
    println!("Longest string is {result}");
}

// fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
//     if x.len() > y.len() {
//         return x
//     }
//     y
// }

fn longest<'a>(x: &'a str, y: &str) -> &'a str {
    x
}
