use camino::Utf8PathBuf;
use std::result;

/// Integration Error
#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error(transparent)]
    Io(#[from] IoError),

    #[error(transparent)]
    Lua(#[from] LuaError),

    #[error(transparent)]
    StdIo(#[from] std::io::Error),
}

/// Error about Lua
#[derive(thiserror::Error, Debug)]
pub enum LuaError {
    #[error("Failed to load a lua file: {0}")]
    FailToLoadLua(#[from] mlua::Error),
}

/// Error about IO
#[derive(thiserror::Error, Debug)]
pub enum IoError {
    #[error("Directory not found: '{0}")]
    DirectoryNotFound(Utf8PathBuf),

    #[error("File not found: '{0}'")]
    FileNotFound(Utf8PathBuf),
}

/// Specialized `Result` type for Error
pub type Result<T> = result::Result<T, Error>;
