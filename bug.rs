fn main() {
    let mut vec = Vec::new();
    vec.push(1);
    vec.push(2);
    vec.push(3);

    let mut iter = vec.iter();

    // This is correct
    println!("1st element: {:?}", iter.next());
    println!("2nd element: {:?}", iter.next());
    println!("3rd element: {:?}", iter.next());

    // This will panic! iter is exhausted.
    println!("4th element: {:?}", iter.next());
}