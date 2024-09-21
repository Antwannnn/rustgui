pub mod encoding{
    pub mod encoding_detector;
}

pub mod handlers{
    pub mod file_handler;
}

use std::sync::Arc;
use iced::executor;
use iced::widget::{button, column, container, horizontal_space, row, text, text_editor};
use iced::{ Element, Length, Application, Settings, Theme, Command };
use handlers::file_handler::{Error, pick_file, save_file, save_file_as_dialog};
use encoding::encoding_detector;



fn main() -> iced::Result {
    return TextEditor::run(Settings::default());
}

#[derive(Default)]
struct TextEditor{
    content: text_editor::Content,
    opened_file: String,
    error: Option<Error>,
}

#[derive(Debug, Clone)]
enum Message {
    Edit(text_editor::Action),
    Open,
    Save,
    SaveAs,
    FileOpened(Result<(Arc<String>, String), Error>),
    FileSaved(Result<(), Error>),
}

impl Application for TextEditor {

    type Message = Message;
    type Theme = Theme;
    type Executor = executor::Default;
    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, Command<Message>) {
        (
            Self{
                content: text_editor::Content::new(),
                opened_file: String::new(),
                error: None,
            }, 

            Command::none()
        )
    }
    

    fn title(&self) -> String {
        String::from("Simple text editor in Rust")
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::Edit(action) => {
                self.content.edit(action);
                Command::none()
            },
            // Handle the file opening
            Message::FileOpened(result) => {
                match result {
                    Ok(text) => {
                        self.content = text_editor::Content::with(&text.0);
                        self.opened_file = text.1;
                    },
                    Err(error) => {
                        self.error = Some(error);
                    }
                }
                Command::none()
            },
            
            Message::FileSaved(result) => {
                match result {
                    Ok(_) => {},
                    Err(error) => {
                        self.error = Some(error);
                    }
                }
                Command::none()
            },
            
            Message::Open => {
                return Command::perform(pick_file(), Message::FileOpened);
            },
            Message::Save => {
                let path = self.opened_file.clone();
                let content = self.content.text().to_string();
                return Command::perform(save_file(path, content), Message::FileSaved);
            },

            Message::SaveAs => {
                return Command::perform(save_file_as_dialog(self.content.text().to_string()), Message::FileSaved);
            },
            
        }

    }

    fn view(&self) -> Element<'_, Message> {
        let input = text_editor(&self.content)
            .on_edit(Message::Edit);

        let position = {
            let (line, column) = self.content.cursor_position();

            text(format!("Ln: {}, Col: {}", line + 1, column + 1))
        };

        let current_file = text(format!("{}", self.opened_file));

        let encoding_type = {
            let encoding = encoding_detector::detect_encoding(self.content.text().as_bytes());
            text(format!("{}", encoding.name()))
        };

        let controls = row![
            button("Open")
                .on_press(Message::Open)    
            ,
            button("Save")
                .on_press(Message::Save)
            ,
            button("Save As")
                .on_press(Message::SaveAs)

        ].spacing(10);

        let status_bar_left = row![current_file].spacing(10);
        let status_bar_right = row![encoding_type, position].spacing(10);

        let status_bar = row![status_bar_left, horizontal_space(Length::Fill), status_bar_right];
        container(column![controls, input, status_bar].spacing(10))
            .padding(10)
            .into()
    }

    fn theme(&self) -> Theme {
        Theme::Dark
    }
}   
