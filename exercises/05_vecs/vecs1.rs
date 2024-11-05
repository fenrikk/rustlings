fn array_and_vec() -> ([i32; 4], Vec<i32>) {
    let a = [10, 20, 30, 40]; // Array

    // Create a vector called `v` which contains the exact same elements as in the array `a`.
    let v = vec![10, 20, 30, 40];

    (a, v)
}

fn main() {
    let (array, vector) = array_and_vec();
    println!("Array: {:?}", array);
    println!("Vector: {:?}", vector);
}