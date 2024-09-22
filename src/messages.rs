// src/messages.rs

use std::path::PathBuf;
use std::sync::Arc;
use crate::handlers::file_handler::Error;
use iced::highlighter::{self};
use iced::widget::text_editor;

#[derive(Debug, Clone)]
pub enum EditorMessage {
    Edit(text_editor::Action),
    Open,
    New,
    Save,
    FileOpened(Result<(Arc<String>, Option<PathBuf>), Error>),
    FileSaved(Result<PathBuf, Error>),
    ThemeChanged(highlighter::Theme),
}
