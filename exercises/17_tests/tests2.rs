// Calculates the power of 2 using a bit shift.
// `1 << n` is equivalent to "2 to the power of n".
fn power_of_2(n: u8) -> u64 {
    1 << n
}

fn main() {
    println!("2^0 = {}", power_of_2(0));
    println!("2^1 = {}", power_of_2(1));
    println!("2^2 = {}", power_of_2(2));
    println!("2^3 = {}", power_of_2(3));
}

#[cfg(test)]
mod tests {
    use super::power_of_2;

    #[test]
    fn you_can_assert_eq() {
        assert_eq!(power_of_2(0), 1);
        assert_eq!(power_of_2(1), 2);
        assert_eq!(power_of_2(2), 4);
        assert_eq!(power_of_2(3), 8);
    }
}