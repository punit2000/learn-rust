
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32
}

impl Rectangle {
    fn calculate_area(&self) -> u32 {
        self.height * self.width
    }

    fn can_hold(&self, other: &Rectangle) -> bool{
        self.width >= other.width && self.height >= other.height
    }

    fn square(side: u32) -> Self {
        Self {
            width: side,
            height: side
        }
    }
}

struct Color(u8, u8, u8);
struct Point(u8, u8, u8);

fn main() {
    let mut user_1 = User {
        email: String::from("punit@gmail.com"),
        username: String::from("punit2000"),
        active: true,
        sign_in_count: 5
    };
    // let user_2 = User {
    //     email: String::from("john@gmail.com"),
    //     username: String::from("john2000"),
    //     active: true,
    //     sign_in_count: 1
    // };
    let user_2 = build_user(
        String::from("john"), String::from("john@gmail.com")
    );
    let user_3 = User {
        email: String::from("another@example.com"),
        ..user_1
    };
    user_1.username = String::from("punpun");
    println!("name of user 1 is {}",user_1.username);
    println!("name of user 2 is {}",user_2.username);
    println!("name of user 3 is {}",user_3.username);

    let red: Color = Color(100, 0, 0);
    set_bg_color(red);

    let point: Point = Point(30, 40, 50);
    move_point(point);


    // let w = 100;
    // let h = 200;

    let rect = Rectangle {
        height: 32,
        width: 50
    };
    let rect2 = Rectangle {
        height: 50,
        width: 50
    };
    let area = dbg!(calculate_area(&rect)); //how will you know which value is height or width ...both are independent values
    //println!("Area of a rectangle with width as {} and height as {} is {area}", rect.width, rect.height);
    dbg!(&rect);
    println!("Area of a rectangle {rect:#?} is {area}");

    println!("Area of a rectangle is {}",rect.calculate_area());
    println!("Area of rect2 is {}",rect2.calculate_area());

    println!("can rect1 hold rect2? {}", rect2.can_hold(&rect));

    let sq: Rectangle = Rectangle::square(5);
    dbg!(sq);

}

fn build_user(username:String, email:String) -> User {
    User {
        username: username,
        email: email,
        active: true,
        sign_in_count: 0
    }
}


fn set_bg_color(color: Color) {
    println!(
        "Setting background colour R={}, G={}, B={}",
        color.0, color.1, color.2
    )
}

fn  move_point(point: Point) {
    println!(
        "The cursor was moved X={}, Y={}, Z={}",
        point.0, point.1, point.2
    )
}

fn calculate_area(rect: &Rectangle) -> u32 {
    rect.width * rect.height
}