#[derive(Debug)]
struct Point {
    x: u64,
    y: u64,
}

struct Dimensions {
    width: u64,
    height: u64,
}

struct ChangeColor(u8, u8, u8);

#[derive(Debug)]
enum Message {
    // TODO: Define the different variants used below.
    Quit,
    Move(Point),
    Resize { width: u64, height: u64 },
    ChangeColor(u8, u8, u8),
    Echo(String),
}

impl Message {
    fn call(&self) {
        println!("{self:?}");
    }
}

fn main() {
    let messages = [
        Message::Resize {
            width: 10,
            height: 30,
        },
        Message::Move(Point { x: 10, y: 15 }),
        Message::Echo(String::from("hello world")),
        Message::ChangeColor(200, 255, 255),
        Message::Quit,
    ];

    for message in &messages {
        message.call();
    }
}
