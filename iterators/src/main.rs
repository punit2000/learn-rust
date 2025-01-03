fn main() {
    let v1 = vec![1, 2, 3];
    let v1_iter = v1.iter();

    for val in v1_iter {
        println!("Got: {val}")
    }

   // let total = v1_iter.sum(); wont work

    let total: Vec<_> = v1.iter().map(|x| x + 1).collect();
    println!("{:?}", total);
}
