fn main() {
    let mut vec = Vec::new();
    vec.push(1);
    vec.push(2);
    vec.push(3);

    let mut iter = vec.iter();

    println!("1st element: {:?}", iter.next());
    println!("2nd element: {:?}", iter.next());
    println!("3rd element: {:?}", iter.next());

    // Correctly handles the possibility of an exhausted iterator
    match iter.next() {
        Some(element) => println!("4th element: {:?}", element),
        None => println!("Iterator exhausted"),
    }
} 