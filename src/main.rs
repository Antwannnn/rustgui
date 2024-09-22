pub mod encoding{
    pub mod encoding_detector;
}

pub mod handlers{
    pub mod file_handler;
}

pub mod ui{pub mod fonts;}

mod messages;

use ui::fonts::{UIFonts, icon};
use std::path::{Path, PathBuf};
use iced::{executor, Font};
use iced::widget::{pick_list, button, column, container, horizontal_space, row, text, text_editor, tooltip};
use iced::{ Element, Length, Application, Settings, Theme, Command };
use handlers::file_handler::{Error, pick_file, save_file};
use encoding::encoding_detector;
use messages::EditorMessage;
use iced::keyboard;
use iced::theme;
use iced::highlighter::{self, Highlighter};


fn main() -> iced::Result {
    return TextEditor::run(Settings{
        default_font: Font::DEFAULT,
        default_text_size: iced::Pixels(17.0),
        fonts: vec![include_bytes!("../fonts/context-icons.ttf").as_slice().into()],
        window: iced::window::Settings{
            size: (1280, 720),
            position: iced::window::Position::Centered,
            ..iced::window::Settings::default()
        },
        ..Settings::default()
    });
}

struct TextEditor{
    content: text_editor::Content,
    path: Option<PathBuf>,
    is_dirty: bool,
    theme: highlighter::Theme,
    error: Option<Error>,
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
                is_dirty: true,
                path: None,
                theme: highlighter::Theme::SolarizedDark,
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
                self.is_dirty = self.is_dirty || action.is_edit();
                self.content.edit(action);
                self.error = None;
                Command::none()
            },

            EditorMessage::New => {
                self.content = text_editor::Content::new();
                self.is_dirty = true;
                self.path = None;
                Command::none()
            },

            EditorMessage::Open => {
                return Command::perform(pick_file(), EditorMessage::FileOpened);
            },
            // Handle the file opening
            EditorMessage::FileOpened(result) => {
                match result {
                    Ok(text) => {
                        self.content = text_editor::Content::with(&text.0);
                        self.path = text.1;
                        self.is_dirty = false;
                    },
                    Err(error) => {
                        self.error = Some(error);
                    }
                }
                Command::none()
            },
            EditorMessage::Save => {
                let content = self.content.text();

                return Command::perform(save_file(self.path.clone(), content), EditorMessage::FileSaved);
            },

            EditorMessage::FileSaved(result) => {
                match result {
                    Ok(path) => {
                        self.path = Some(path);
                        self.is_dirty = false;
                    },
                    Err(error) => {
                        self.error = Some(error);
                    }
                }
                Command::none()
            },

            EditorMessage::ThemeChanged(theme) => {
                self.theme = theme;
                Command::none()
            }
            
            
        }

    }

    fn subscription(&self) -> iced::Subscription<Self::Message> {
        keyboard::on_key_press(|key_code, modifiers|{
            if modifiers.command() {
                match key_code {
                    keyboard::KeyCode::S => {
                        return Some(EditorMessage::Save);
                    },
                    keyboard::KeyCode::O => {
                        return Some(EditorMessage::Open);
                    },
                    keyboard::KeyCode::N => {
                        return Some(EditorMessage::New);
                    },
                    _ => {}
                }
            }
            None
        })
    }

    fn view(&self) -> Element<'_, EditorMessage> {

        

        let input = text_editor(&self.content)
            .on_edit(EditorMessage::Edit)
            .highlight::<Highlighter>(highlighter::Settings{
                theme: self.theme,
                extension: self.path.as_ref().and_then(|ext| ext.extension()?.to_str()).unwrap_or("txt").to_string(), 
            }, |highlight, _theme|{
                highlight.to_format()
            });
        
        let controls = row![
            // New button
            action(icon(UIFonts::ContextIcons, '\u{E800}'), "Create a new file", Some(EditorMessage::New)),
            // Save button
            action(icon(UIFonts::ContextIcons, '\u{E801}'), "Save the current file", self.is_dirty.then_some(EditorMessage::Save)),
            // Open button
            action(icon(UIFonts::ContextIcons, '\u{F115}'), "Open an existing file", Some(EditorMessage::Open)),
            horizontal_space(Length::Fill),
            pick_list(
                &highlighter::Theme::ALL[..],
                Some(self.theme),
                EditorMessage::ThemeChanged
            )
    
        ].spacing(10);

        let status_bar = {

            let status = if let Some(error) = self.error.as_ref() {
                text(format!("Error: {:?}", error))
            } else {
                match self.path.as_deref().and_then(Path::to_str) {
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
        if self.theme.is_dark(){
            return Theme::Dark;
        } else{
            return Theme::Light;
        }
    }
}

fn action<'a>(content: Element<'a, EditorMessage>, label: &str, on_press: Option<EditorMessage>) -> Element<'a, EditorMessage> {
    let is_disabled = on_press.is_none();
    tooltip(button(container(content)
        .width(30)
        .center_x())
    .on_press_maybe(on_press)
    .style(
        if is_disabled {
            theme::Button::Secondary
        } else {
            theme::Button::Primary
        }
    )        
    .padding([5,7]), label, tooltip::Position::FollowCursor)
    .style(theme::Container::Box)
    .into()
}



