mod delicious_snacks {
    pub use self::fruits::PEAR;
    pub use self::veggies::CUCUMBER;

    mod fruits {
        pub const PEAR: &str = "Pear";
        pub const APPLE: &str = "Apple";
    }

    mod veggies {
        pub const CUCUMBER: &str = "Cucumber";
        pub const CARROT: &str = "Carrot";
    }
}

fn main() {
    println!(
        "favorite snacks: {} and {}",
        delicious_snacks::PEAR,
        delicious_snacks::CUCUMBER
    );
}