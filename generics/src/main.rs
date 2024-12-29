#[derive(Debug)]
struct Point<T, U> {
    x: T,
    y: U

}

impl <T, U> Point<T,U> {
    fn new(x:T, y: U) -> Self {
        Self { x: x, y: y }
    } 

    fn mixup<X, Y>(self, point: Point<X,Y>) -> Point<T, Y> {
        Point {
            x: self.x,
            y: point.y
        }
    } 
}

impl  Point<f64, f64> {
    fn calculate_distance(&self) -> f64 {
        4.0
    }
}

fn main() {
    let list = vec![1,2,7,11,10];
    let list_2 = vec![1.1,2.2,1.4,1.0,1.5];
    let l_2 = largest(&list_2);
    let l = largest(&list);
    println!("largest number is {l}");
    println!("largest number of list_2 is {l_2}");

    let point_a = Point {x:10, y:1.2};
    let point_b =  Point::new(6.2, 5.0); 

    point_b.calculate_distance();

    let point_c = point_a.mixup(point_b);

    println!("point c {:?}", point_c);
}

fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
    let mut result = &list[0];
    for item in list {
        if item > result {
            result = item;
        }
    }

    result
}
