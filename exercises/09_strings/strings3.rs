fn trim_me(input: &str) -> &str {
    input.trim()
}

fn compose_me(input: &str) -> String {
    format!("{} world!", input)
}

fn replace_me(input: &str) -> String {
    input.replace("cars", "balloons")
}

fn main() {
    let trimmed = trim_me("   Hello!   ");
    println!("Trimmed: '{}'", trimmed);

    let composed = compose_me("Hello");
    println!("Composed: '{}'", composed);

    let replaced = replace_me("I think cars are cool");
    println!("Replaced: '{}'", replaced);
}