fn main() {
    let mut vec = Vec::new();
    vec.push(1);
    vec.push(2);
    vec.push(3);

    // This will cause a panic if the vector is empty
    println!("First element: {}", vec[0]);
}