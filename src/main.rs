use camino::Utf8PathBuf;
use my_sync::lua::LuaLoader;

fn main() {
    // make path of the target directory incluedes lua files
    let mut target_dir = Utf8PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    target_dir.push("example");

    // lua loading
    let loader = LuaLoader::from_buf(target_dir).expect("Failed to initialization for loading Lua");
    loader.load_package().expect("Failed to load functions");

    println!("--- lua ---");
    loader.load("sample.lua").expect("Failed to load lua");
    println!("-----------");
    println!("Successed to load Lua file");
}
