struct Rectangle {
    width: i32,
    height: i32,
}

impl Rectangle {
    fn new(width: i32, height: i32) -> Self {
        if width <= 0 || height <= 0 {
            panic!("Rectangle width and height must be positive");
        }

        Rectangle { width, height }
    }
}

fn main() {
    let rect = Rectangle::new(10, 20);
    println!("Width: {}", rect.width);
    println!("Height: {}", rect.height);

    // Attempting to create a rectangle with negative dimensions
    // This will panic
    let _invalid_rect = Rectangle::new(-5, 10);
}

#[cfg(test)]
mod tests {
    use super::Rectangle;

    #[test]
    fn correct_width_and_height() {
        let rect = Rectangle::new(10, 20);
        assert_eq!(rect.width, 10);
        assert_eq!(rect.height, 20);
    }

    #[test]
    #[should_panic]
    fn negative_width() {
        let _rect = Rectangle::new(-10, 10);
    }

    #[test]
    #[should_panic]
    fn negative_height() {
        let _rect = Rectangle::new(10, -10);
    }
}