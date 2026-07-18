use crate::error::{IoError, LuaError, Result};
use crate::lua;
use camino::Utf8PathBuf;
use mlua::{self, Lua};
use std::fs;

/// ユーザー設定ファイル(luaファイル)を読み込むための構造体
#[allow(dead_code)]
pub struct LuaLoader {
    lua: Lua,
    directory: Utf8PathBuf,
}

#[allow(dead_code)]
impl LuaLoader {
    pub fn new(directory: Utf8PathBuf) -> Result<Self> {
        if !directory.exists() {
            Err(IoError::DirectoryNotFound(directory).into())
        } else {
            Ok(LuaLoader {
                lua: Lua::new(),
                directory,
            })
        }
    }

    pub fn from(dir: &(impl AsRef<str> + ?Sized)) -> Result<Self> {
        LuaLoader::new(Utf8PathBuf::from(dir))
    }

    pub fn from_buf(dir: Utf8PathBuf) -> Result<Self> {
        LuaLoader::new(dir)
    }

    pub fn load_functions(&self) -> Result<()> {
        let result = lua::functions::set_functions(&self.lua);
        if let Err(err) = result {
            Err(LuaError::FailedToLoad(err).into())
        } else {
            Ok(())
        }
    }

    pub fn load(&self, file: &(impl AsRef<str> + ?Sized)) -> Result<bool> {
        let file_path = self.directory.join(Utf8PathBuf::from(file));
        if !file_path.exists() {
            Err(IoError::FileNotFound(file_path).into())
        } else {
            let source = fs::read_to_string(file_path)?;
            self.lua
                .load(source)
                .exec()
                .map_err(LuaError::FailedToLoad)?;
            Ok(true)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::error::{Error, IoError, LuaError};
    use camino::Utf8PathBuf;

    #[test]
    fn test_from_exists_dir() {
        let loader = LuaLoader::from("/tmp");
        assert!(loader.is_ok());
    }

    #[test]
    fn test_from_noexists_dir() {
        let loader = LuaLoader::from("/noexists");
        assert!(loader.is_err());
    }

    #[test]
    fn test_load_noexists_file() {
        let loader = LuaLoader::from("/tmp").unwrap();
        let result = loader.load("noexists.lua");
        assert!(result.is_err());

        let err = result.unwrap_err();
        if let Error::Io(IoError::FileNotFound(path)) = err {
            assert_eq!(path, Utf8PathBuf::from("/tmp/noexists.lua"));
        }
    }

    #[test]
    fn test_load_invalid_lua() {
        let test_lua_dir = Utf8PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/lua/");
        let loader = LuaLoader::from_buf(test_lua_dir).unwrap();
        let result = loader.load("invalid.lua");
        assert!(result.is_err());

        let err = result.unwrap_err();
        assert!(matches!(err, Error::Lua(LuaError::FailedToLoad(_))));
    }
}
