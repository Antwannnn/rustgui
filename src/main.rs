pub mod encoding{
    pub mod encoding_detector;
}

pub mod handlers{
    pub mod file_handler;
}

use std::path::{Path, PathBuf};
use std::sync::Arc;
use iced::executor;
use iced::widget::{button, column, container, horizontal_space, row, text, text_editor, Row};
use iced::{ Element, Length, Application, Settings, Theme, Command };
use handlers::file_handler::{Error, pick_file, save_file, save_file_as_dialog};
use encoding::encoding_detector;


fn main() -> iced::Result {
    return TextEditor::run(Settings{
        window: iced::window::Settings{
            size: (1280, 720),
            position: iced::window::Position::Centered,
            ..iced::window::Settings::default()
        },
        ..Settings::default()
    });
}

#[derive(Default)]
struct TextEditor{
    content: text_editor::Content,
    opened_file: Option<PathBuf>,
    modified : bool,
    error: Option<Error>,
}

#[derive(Debug, Clone)]
enum EditorMessage {
    Edit(text_editor::Action),
    Open,
    New,
    Save,
    SaveAs,
    FileOpened(Result<(Arc<String>, Option<PathBuf>), Error>),
    FileSaved(Result<(), Error>),
}


impl Application for TextEditor {

    type Message = EditorMessage;
    type Theme = Theme;
    type Executor = executor::Default;
    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, Command<EditorMessage>) {
        (
            Self{
                content: text_editor::Content::new(),
                modified: false,
                opened_file: None,
                error: None,
            }, 

            Command::none()
        )
    }
    

    fn title(&self) -> String {
        String::from("Simple text editor in Rust")
    }

    fn update(&mut self, message: EditorMessage) -> Command<EditorMessage> {
        match message {
            EditorMessage::Edit(action) => {
                if self.modified == false {
                    self.modified = true;
                }
                self.content.edit(action);
                self.error = None;
                Command::none()
            },
            // Handle the file opening
            EditorMessage::FileOpened(result) => {
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
            
            EditorMessage::FileSaved(result) => {
                match result {
                    Ok(_) => {},
                    Err(error) => {
                        self.error = Some(error);
                    }
                }
                Command::none()
            },

            
            EditorMessage::Open => {
                return Command::perform(pick_file(), EditorMessage::FileOpened);
            },
            EditorMessage::Save => {
                let path = self.opened_file.clone();
                let content = self.content.text().to_string();
                return Command::perform(save_file(path.unwrap(), content), EditorMessage::FileSaved);
            },

            EditorMessage::SaveAs => {
                return Command::perform(save_file_as_dialog(self.content.text().to_string()), EditorMessage::FileSaved);
            },

            EditorMessage::New => {
                self.content = text_editor::Content::new();
                self.modified = false;
                self.opened_file = None;
                Command::none()
            },
            
            
        }

    }

    fn view(&self) -> Element<'_, EditorMessage> {

        let input = text_editor(&self.content)
            .on_edit(EditorMessage::Edit);

        
        let controls = row![
            button("New")
                .on_press(EditorMessage::New),
            button("Open")
                .on_press(EditorMessage::Open),
            button("Save")
                .on_press(EditorMessage::Save),
            button("Save As")
                .on_press(EditorMessage::SaveAs)
    
        ].spacing(10);

        let status_bar = {

            let status = if let Some(error) = self.error.as_ref() {
                text(format!("Error: {:?}", error))
            } else {
                match self.opened_file.as_deref().and_then(Path::to_str) {
                    Some(file) => text(format!("Editing: {}", file)),
                    None => text("New File"),
                }
            };   

    
            let encoding_type = {
                let encoding = encoding_detector::detect_encoding(self.content.text().as_bytes());
                text(format!("{}", encoding.name()))
            };

            let position = {
                let (line, column) = self.content.cursor_position();
    
                text(format!("Ln: {}, Col: {}", line + 1, column + 1))
            };
            
            row![status, horizontal_space(Length::Fill), row![encoding_type, position].spacing(10)]
        };
        container(column![controls, input, status_bar].spacing(10))
            .padding(10)
            .into()
    }

    fn theme(&self) -> Theme {
        Theme::Dark
    }
}   
