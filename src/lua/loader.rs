use crate::error::{IoError, LuaError, Result};
use camino::Utf8PathBuf;
use mlua::Lua;
use std::fs;

/// ユーザー設定ファイル(luaファイル)を読み込むための構造体
#[allow(dead_code)]
pub struct LuaLoader {
    directory: Utf8PathBuf,
}

#[allow(dead_code)]
impl LuaLoader {
    pub fn new(directory: Utf8PathBuf) -> Self {
        Self { directory }
    }

    pub fn from(dir: &(impl AsRef<str> + ?Sized)) -> Result<Self> {
        let path = Utf8PathBuf::from(dir);
        if !path.exists() {
            Err(IoError::DirectoryNotFound(path).into())
        } else {
            Ok(LuaLoader::new(path))
        }
    }

    pub fn load(&self, file: &(impl AsRef<str> + ?Sized)) -> Result<bool> {
        let file_path = self.directory.join(Utf8PathBuf::from(file));
        if !file_path.exists() {
            Err(IoError::FileNotFound(file_path).into())
        } else {
            let source = fs::read_to_string(file_path)?;
            let lua = Lua::new();
            lua.load(source).exec().map_err(LuaError::FailToLoadLua)?;
            Ok(true)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_exists_dir() {
        let loader = LuaLoader::from("/tmp");
        assert!(loader.is_ok());
    }

    #[test]
    fn test_from_noexists_dir() {
        let loader = LuaLoader::from("/aaaaaaaaaaa");
        assert!(loader.is_err());
    }
}
