use std::{fs::File, io::Read};

fn main() {
    println!("cargo:rerun-if-changed=build/build.rs");
    use std::time::SystemTime;
    let now: String = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .ok()
        .unwrap()
        .as_secs()
        .to_string();

    println!("cargo:rustc-env=H10_BUILT_AT={now}");

    let mut file = File::open("Cargo.toml").unwrap();
    let mut buf = "".to_string();
    file.read_to_string(&mut buf).unwrap();

    let repo = buf
        .lines()
        .filter(|s| s.contains("repository"))
        .next()
        .unwrap()
        .split("=")
        .nth(1)
        .unwrap()
        .replace(r#"""#, "");

    println!("cargo:rustc-env=CARGO_PKG_REPOSITORY={}", repo.trim());

    let desc = buf
        .lines()
        .filter(|s| s.contains("description"))
        .next()
        .unwrap()
        .split("=")
        .nth(1)
        .unwrap()
        .replace(r#"""#, "");

    println!("cargo:rustc-env=CARGO_PKG_DESCRIPTION={}", desc.trim());
}
