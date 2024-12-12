fn main() {
    let mut vec = Vec::new();
    vec.push(1);
    vec.push(2);
    vec.push(3);

    // Check if the vector is empty before accessing elements
    if !vec.is_empty() {
        println!("First element: {}", vec[0]);
    } else {
        println!("Vector is empty!");
    }

    // Or use the get method which returns an Option
    match vec.get(0) {
        Some(first) => println!("First element: {}", first),
        None => println!("Vector is empty!")
    }
} 