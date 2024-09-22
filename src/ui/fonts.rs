// Module: UI

use crate::messages::EditorMessage;
use iced::{Element, Font};
use iced::widget::{text};

pub enum UIFonts {
    ContextIcons,
}

impl UIFonts {
    pub fn name(&self) -> &'static str {
        match self {
            UIFonts::ContextIcons => "context-icons",
        }
    }
}

pub fn icon<'a>(icon: UIFonts, codepoint: char) -> Element<'a, EditorMessage> {

    let icon_font: Font = Font::with_name(icon.name());

    text(codepoint).font(icon_font).into()
}

