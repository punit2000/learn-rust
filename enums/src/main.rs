use std::{f64::MAX_10_EXP, i32};

#[derive(Debug)]
enum IpAddrKind {
    v4(u8,u8,u8,u8),
    v6(String)
}

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}
// struct IpAddress {
//     address: String,
//     kind: IpAddrKind
// }

// impl IpAddress {
//     fn new(address: &str) -> Self {
//         Self {
//             address: address.to_string(),
//             kind: IpAddrKind::v4
//         }
//     }
// }


enum Message {
    Quit,
    Move {x: i32, y: i32},
    Write(String),
    ChangeColor(i32, i32, i32),
}
fn main() {
    // let four = IpAddrKind::v4;

    // let google_address = IpAddress {
    //     address: String::from("1.2.3.4"),
    //     kind: IpAddrKind::v4
    // };
    // let google_address = IpAddress::new("1.2.3.4");
    let home = IpAddrKind::v4(127,0,0,1);
    let loopback = IpAddrKind::v6(String::from("::1"));
    route(home);
    route(loopback);

    let m: Message = Message::Write(String::from("hello"));

    // let op: Option<i32> = Some(1);
    let op: Option<i32> = None;
    let x = 2;

    let sum = x + op.unwrap_or(0);
    println!("Sum is {sum}");

    let coin = Coin::Penny;
    let coin2 = Coin::Quarter(UsState::Alabama);
    println!("Value is {}", value_in_cents(coin));
    println!("Value is {}", value_in_cents(coin2));

    println!("Add result = {}", add(50, Some(90)));
    println!("Add result = {}", add(50, None));

    let dice_roll = 6;
    match dice_roll {
        3 => println!("You got a fancy hat 🎩"),
        6 => println!("Your fancy hat is removed 🎩❌"),
        //other => println!("Move {} steps", other), //both correct
        _ => println!("Move {dice_roll} steps")
        // _ => ()
    };

    let config_max = Some(3_u8);

    if let Some(max) = config_max {
        println!("The maximum is configured to be {max}");
    }
    // match config_max {
    //     Some(max) => println!("the maximum is configured to be {max}"),
    //     None => ()
    // }

}

// fn route(ip: &str, kind: IpAddrKind){
//     println!("Routing request to IP {ip} of kind {kind:?}")
// }

// fn route(ip: IpAddress){
//     println!("Routing request to IP {} of kind {:?}", ip.address, ip.kind)
// }

fn route(ip: IpAddrKind){
    println!("Routing request to IP {:?}", ip)
}

fn add(num: i32, num2: Option<i32>) -> i32 {
    match num2 {
        Some(i) => num + i,
        None => num,
    }
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("This is a penny");
            1
        },
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(UsState::Alaska) => {
            println!("Hello from Alaska");
            25
        }, 
        Coin::Quarter(state) => {
            println!("Got Q of {:?}", state);
            25
        },
    }
}