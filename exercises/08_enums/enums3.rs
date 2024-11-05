struct Point {
    x: u64,
    y: u64,
}

enum Message {
    Resize { width: u64, height: u64 },
    Move(Point),
    Echo(String),
    ChangeColor(u8, u8, u8),
    Quit,
}

struct State {
    width: u64,
    height: u64,
    position: Point,
    message: String,
    // RGB color composed of red, green and blue.
    color: (u8, u8, u8),
    quit: bool,
}

impl State {
    fn resize(&mut self, width: u64, height: u64) {
        self.width = width;
        self.height = height;
    }

    fn move_position(&mut self, point: Point) {
        self.position = point;
    }

    fn echo(&mut self, s: String) {
        self.message = s;
    }

    fn change_color(&mut self, red: u8, green: u8, blue: u8) {
        self.color = (red, green, blue);
    }

    fn quit(&mut self) {
        self.quit = true;
    }

    fn process(&mut self, message: Message) {
        match message {
            Message::Resize { width, height } => self.resize(width, height),
            Message::Move(point) => self.move_position(point),
            Message::Echo(s) => self.echo(s),
            Message::ChangeColor(red, green, blue) => self.change_color(red, green, blue),
            Message::Quit => self.quit(),
        }
    }
}

fn main() {
    let mut state = State {
        width: 100,
        height: 100,
        position: Point { x: 0, y: 0 },
        message: String::from("Initial message"),
        color: (0, 0, 0),
        quit: false,
    };

    println!("Initial state:");
    println!("Size: {}x{}", state.width, state.height);
    println!("Position: ({}, {})", state.position.x, state.position.y);
    println!("Message: {}", state.message);
    println!("Color: {:?}", state.color);
    println!("Quit: {}", state.quit);
    println!("\n---Processing messages---\n");

    // Resize window
    state.process(Message::Resize {
        width: 800,
        height: 600,
    });
    println!("After resize: {}x{}", state.width, state.height);

    // Move window
    state.process(Message::Move(Point { x: 10, y: 20 }));
    println!("After move: position ({}, {})", state.position.x, state.position.y);

    // Change message
    state.process(Message::Echo(String::from("Hello, Rust!")));
    println!("After echo: message is \"{}\"", state.message);

    // Change color to purple
    state.process(Message::ChangeColor(255, 0, 255));
    println!("After color change: RGB color is {:?}", state.color);

    // Quit
    state.process(Message::Quit);
    println!("After quit: quit state is {}", state.quit);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_match_message_call() {
        let mut state = State {
            width: 0,
            height: 0,
            position: Point { x: 0, y: 0 },
            message: String::from("hello world"),
            color: (0, 0, 0),
            quit: false,
        };

        state.process(Message::Resize {
            width: 10,
            height: 30,
        });
        state.process(Message::Move(Point { x: 10, y: 15 }));
        state.process(Message::Echo(String::from("Hello world!")));
        state.process(Message::ChangeColor(255, 0, 255));
        state.process(Message::Quit);

        assert_eq!(state.width, 10);
        assert_eq!(state.height, 30);
        assert_eq!(state.position.x, 10);
        assert_eq!(state.position.y, 15);
        assert_eq!(state.message, "Hello world!");
        assert_eq!(state.color, (255, 0, 255));
        assert!(state.quit);
    }
}
