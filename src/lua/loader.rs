use camino::Utf8PathBuf;
use std::io::{Error, ErrorKind, Result};

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
            Err(Error::new(
                ErrorKind::NotFound,
                format!("Directory not found: '{}'", path),
            ))
        } else {
            Ok(LuaLoader::new(path))
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
