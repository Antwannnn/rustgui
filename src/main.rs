use iced::widget::text;
use iced::{ Element, Sandbox, Settings };

fn main() {
    
}

struct Editor;

#[derive(Debug)]
enum Message{}

impl Sandbox for Editor {

    type Message = Message;

    fn new() -> Self {
        Self
    }

    fn title(&self) -> String {
        String::from("A cool text editor")
    }

    fn update(&mut self, message: Message) {
        match message {

        }
    }

    fn view(&mut self) -> Element<'_, Message> {
        text("Hello, world!").into();
    }

}