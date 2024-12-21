use core::num;

fn main() {
    const A: i32 = 54;
    const PI: f32 = 3.14;
    let m = 2.5; //floating variables are always signed
    let x = 254u8;
    let a = 254_u8;
    let s = 1_00_000;
    let y = 0b1010; //binary
    let z = b'A'; //byte
    //let c: u8 = random_number() + 100; //integer overflow..goes into panic mode.. "-- release flag wont go into panic mode ..does two's complement wrapping"
    let d: u8 = random_number().wrapping_add(57); //wont cause erros in debug mode
    // let e = match random_number().checked_add(57) {
    //     Some(num) => num,
    //     None => {
    //         println!("Cannot add");
    //         return;
    //     }
    // };
    println!("A: {A}, PI: {PI}, x: {x}, a: {a}, s: {s}, y: {y}, z: {z}");
    // output:
    // A: 54, PI: 3.14, x: 254, a: 254, s: 100000, y: 10, z: 65, c: 44 (256+44 = 300)
    // _ acts as a visual separator
    let x: f32 = 5_f32/2_f32; //will value in decimals
    println!("X is {x}");
    let c = 'z';
    let z: char = 'ℤ'; // with explicit type annotation
    let heart_eyed_cat = '😻';

    //compound types i.e having multiple values

    let tup: (i32, f64, i32) = (500, 6.4, 1);
    let (x,y,z) = tup;
    println!("x is {x}, y is {y}, z is {z}");
    println!("0th value is {}", tup.0);
    println!("1st value is {}", tup.1);
    println!("2nd value is {}", tup.2);
    // tuple with no value is called as unit

    //array having multiple values but of same types ..fixed in length
    let arr: [i32; 5] = [1,2,3,4,5];
    let b = [10;5]; //[10,10,10,10,10]
}

fn random_number() -> u8 {
    200
}

