fn is_even(n: i64) -> bool {
    n % 2 == 0
}

fn main() {
    println!("4 is even: {}", is_even(4));
    println!("7 is even: {}", is_even(7));
}

#[cfg(test)]
mod tests {
    use super::is_even;

    #[test]
    fn you_can_assert() {
        assert!(is_even(4));
        assert!(!is_even(7));
    }
}