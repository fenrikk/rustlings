fn animal_habitat(animal: &str) -> &str {
    let identifier = match animal {
        "crab" => 1,
        "gopher" => 2,
        "snake" => 3,
        _ => 4,
    };

    match identifier {
        1 => "Beach",
        2 => "Burrow",
        3 => "Desert",
        _ => "Unknown",
    }
}

fn main() {
    // You can optionally experiment here.
    println!("Animal habitat: {}", animal_habitat("crab"));
    println!("Animal habitat: {}", animal_habitat("gopher"));
    println!("Animal habitat: {}", animal_habitat("snake"));
    println!("Animal habitat: {}", animal_habitat("dinosaur"));
}

// Don't change the tests!
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gopher_lives_in_burrow() {
        assert_eq!(animal_habitat("gopher"), "Burrow")
    }

    #[test]
    fn snake_lives_in_desert() {
        assert_eq!(animal_habitat("snake"), "Desert")
    }

    #[test]
    fn crab_lives_on_beach() {
        assert_eq!(animal_habitat("crab"), "Beach")
    }

    #[test]
    fn unknown_animal() {
        assert_eq!(animal_habitat("dinosaur"), "Unknown")
    }
}
