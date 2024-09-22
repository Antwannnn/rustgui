
use std::io;
use std::path::Path;
use std::sync::Arc;
use io::ErrorKind;
use std::path::PathBuf;
use tokio;


#[derive(Debug, Clone)]
pub enum Error {
    DialogClosed,
    IO(ErrorKind)
}

pub async fn pick_file() -> Result<(Arc<String>, Option<PathBuf>), Error> {
    let file_picker_handle = rfd::AsyncFileDialog::new()
        .set_title("Choose a text file to open in the editor")
        .pick_file()
        .await
        .ok_or(Error::DialogClosed)?;

    let loaded_file = load_file(file_picker_handle.path()).await?;

    let path_buffer: PathBuf = file_picker_handle.path().into();

    Ok((loaded_file, path_buffer.into()))
}

pub async fn load_file(path: impl AsRef<Path>) -> Result<Arc<String>, Error> {
    tokio::fs::read_to_string(path)
        .await
        .map(Arc::new)
        .map_err(|err| err.kind())
        .map_err(Error::IO)

}

pub async fn save_file(path: Option<PathBuf>, content: String) -> Result<PathBuf, Error> {
    let path = if let Some(path) = path {
            path
        } else {
            rfd::AsyncFileDialog::new()
                .set_title("Choose a location to save the file")
                .save_file()
                .await
                .ok_or(Error::DialogClosed)?
                .path()
                .to_path_buf()
        };

    tokio::fs::write(&path, content)
        .await
        .map_err(|err| err.kind())
        .map_err(Error::IO)?;

    Ok(path)
}

