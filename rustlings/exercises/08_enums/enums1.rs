#[derive(Debug)]
enum Message {
    // TODO: Implement the message variant types based on their usage below.
    Resize,
    Move,
    Echo,
    ChangeColor,
    Quit,
}
fn main() {
    println!("{:?}", Message::Resize);
    println!("{:?}", Message::Move);
    println!("{:?}", Message::Echo);
    println!("{:?}", Message::ChangeColor);
    println!("{:?}", Message::Quit);
}
