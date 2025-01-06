 use crate::List::{Cons, Nil};

 use std::{any::type_name, ops::Deref, rc::Rc};

// enum List {
//     Cons(i32, Box<List>),
//     Nil
// }

enum  List {
    Cons(i32, Rc<List>),
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


struct CustomSmartPointer {
    data:String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping customsmartpointer with data `{}`!",self.data);
    }
}

fn main() {
    let b = Box::new(5);
    println!("{}", b);

    // let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));

    let x = 5;
    // let y: &i32 = &x;
    // let y = Box::new(x);

    let y = MyBox::new(x);

    println!("x: {}", x);
    // println!("y: {}", *y);

    assert_eq!(5,x);
    assert_eq!(5,*y);

    let name = MyBox::new(String::from("Rust"));
    hello(&name);

    let c = CustomSmartPointer {
        data: String::from("my stuff",)
    };

    let d = CustomSmartPointer {
        data: String::from("other stuff")
    };

    println!("Custom Smart pointers created!!");
    drop(c);
    println!("Custom Smart pointers dropped before the end of main"); 


    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    println!("count after creating a = {}", Rc::strong_count(&a));

    let b = Cons(3,Rc::clone(&a));
    println!("count after creating b = {}", Rc::strong_count(&a));
    
    {
    let c = Cons(4, Rc::clone(&a));   
    println!("count after creating c = {}", Rc::strong_count(&a));
    }

    println!("count after c goes out of scope = {}", Rc::strong_count(&a));
}

fn hello(name: &str) {
    println!("Hello, {name}");
}