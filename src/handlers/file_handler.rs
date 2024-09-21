
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

pub async fn pick_file() -> Result<(Arc<String>, String), Error> {
    let file_picker_handle = rfd::AsyncFileDialog::new()
        .set_title("Choose a text file to open in the editor")
        .pick_file()
        .await
        .ok_or(Error::DialogClosed)?;

    let loaded_file = load_file(file_picker_handle.path()).await?;

    let path_buffer: PathBuf = file_picker_handle.path().into();
    let file_path = path_buffer.to_str().unwrap().to_string();

    Ok((loaded_file, file_path))
}

pub async fn save_file_as_dialog(content: String) -> Result<(), Error> {
    let file_saver_handle = rfd::AsyncFileDialog::new()
        .set_title("Choose where to save the file")
        .save_file()
        .await
        .ok_or(Error::DialogClosed)?;

    save_file(file_saver_handle.path(), content).await?;

    Ok(())
}

pub async fn load_file(path: impl AsRef<Path>) -> Result<Arc<String>, Error> {
    tokio::fs::read_to_string(path)
        .await
        .map(Arc::new)
        .map_err(|err| err.kind())
        .map_err(Error::IO)

}

pub async fn save_file(path: impl AsRef<Path>, content: String) -> Result<(), Error> {
    tokio::fs::write(path, content)
        .await
        .map_err(|err| err.kind())
        .map_err(Error::IO)
}

