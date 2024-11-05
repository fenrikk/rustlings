fn vec_loop(input: &[i32]) -> Vec<i32> {
    let mut output = Vec::new();

    for element in input {
        output.push(element * 2);
    }

    output
}

fn vec_map(input: &[i32]) -> Vec<i32> {
    input.iter().map(|element| element * 2).collect()
}

fn main() {
    let input = [2, 4, 6, 8, 10];

    println!("vec_loop output: {:?}", vec_loop(&input));
    println!("vec_map output: {:?}", vec_map(&input));
}