use camino::Utf8PathBuf;
use my_sync::lua::LuaLoader;

fn main() {
    let mut target_dir = Utf8PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    target_dir.push("example");
    let loader = LuaLoader::from_buf(target_dir).expect("Failed to initialization for loading Lua");
    let result = loader.load("sample.lua");
    if result.is_ok() {
        println!("Successed to load Lua file");
    } else {
        println!("Failed to load Lua file");
    }
}
