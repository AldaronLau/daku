use std::{
    env,
    fs::{self, File},
    process::Command,
};

use cargo_toml::Manifest;
use yansi::Paint;

const CARGO_BUILD_RUSTFLAGS: &str = concat!(
    r#" --remap-path-prefix=$PWD=_"#,
    r#" --remap-path-prefix=$HOME/.local/lib/cargo=-"#,
    r#" --remap-path-prefix=$HOME/.local/lib/rustup=+"#,
    r#" --remap-path-prefix=$HOME=~"#,
    r#" --remap-path-prefix=$HOME/.cargo/registry/src/=%"#,
    r#" --cfg"#,
    r#" daku"#,
    r#" -C"#,
    r#" link-args=-zstack-size=32768"#,
);

fn main() {
    let plugin = "cargo-daku".green().bold();
    let plugin = format!("  {plugin}");

    println!("{plugin} Setting up…");

    let current_dir = env::current_dir().expect("Couldn't get current dir");
    let manifest = Manifest::from_path(current_dir.join("Cargo.toml")).expect("Failed to find Cargo.toml");
    let package = manifest.package.as_ref().expect("Not a package");
    let name: &str = package.name.as_ref();
    let wasm_file = &format!("./target/bin/{name}.wasm");

    fs::create_dir_all("./target").expect("Failed to create target dir");

    println!("{plugin} Compiling \"{name}\"…");

    let target = current_dir.join("target");
    let path = {
        let mut path = env::var_os("PATH").expect("No path!");
        path.push(":");
        path.push(current_dir);
        path.push("/target/bin");

        path
    };
    let exit_status = Command::new("cargo")
        .arg("install")
        .arg("--path")
        .arg(".")
        .arg("--root")
        .arg(target)
        .arg("--target")
        .arg("wasm32-unknown-unknown")
        .env("CARGO_BUILD_RUSTFLAGS", CARGO_BUILD_RUSTFLAGS)
        .env("PATH", path)
        .spawn()
        .expect("failed to execute cargo")
        .wait()
        .expect("failed to wait for cargo");

    if !exit_status.success() {
        panic!("Command failed");
    }

    println!("{plugin} Snipping panicking code…");

    let exit_status = Command::new("wasm-snip")
        .arg(wasm_file)
        .arg("--snip-rust-panicking-code")
        .arg("-o")
        .arg(wasm_file)
        .arg("--")
        .arg("main")
        .spawn()
        .expect("failed to execute cargo")
        .wait()
        .expect("failed to wait for cargo");

    if !exit_status.success() {
        panic!("Command failed");
    }

    println!("{plugin} Optimizing…");

    let exit_status = Command::new("wasm-opt")
        .arg(wasm_file)
        .arg("-o")
        .arg(wasm_file)
        .arg("-Os")
        .spawn()
        .expect("failed to execute cargo")
        .wait()
        .expect("failed to wait for cargo");

    if !exit_status.success() {
        panic!("Command failed");
    }

    let bytes = File::open(wasm_file)
        .expect("Failed to open file")
        .metadata()
        .expect("Failed to get file metadata")
        .len();

    println!("{plugin} Done! ({bytes} bytes)");
}
