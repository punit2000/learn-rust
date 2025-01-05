 use crate::List::{Cons, Nil};

 use std::ops::Deref;

enum List {
    Cons(i32, Box<List>),
    Nil
}

struct  MyBox<T>(T);

impl <T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl <T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

fn main() {
    let b = Box::new(5);
    println!("{}", b);

    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));

    let x = 5;
    // let y: &i32 = &x;
    // let y = Box::new(x);

    let y = MyBox::new(x);

    println!("x: {}", x);
    // println!("y: {}", *y);

    assert_eq!(5,x);
    assert_eq!(5,*y);
}
