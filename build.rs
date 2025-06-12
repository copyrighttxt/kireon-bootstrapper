use std::fs;
use std::env;
use winres::WindowsResource;
fn main() {
    // Get the current build date and time
    let build_date = chrono::Utc::now().format("%Y-%m-%d %H:%M:%S UTC").to_string();

    // Write the build date to a file
    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_path = format!("{}/build_date.txt", out_dir);
    fs::write(&dest_path, &build_date).unwrap();

    if env::var_os("CARGO_CFG_WINDOWS").is_some() {
        // Add a version resource to the executable
        let mut res = WindowsResource::new();
        res.set_icon("assets/Bootstrapper.ico");
        res.set_language(0x0409); // US English
        res.set("FileVersion", env!("CARGO_PKG_VERSION"));
        res.set("FileDescription", "Kireon Windows Bootstrapper");
        res.set("ProductName", "Kireon Bootstrapper");
        res.set("ProductVersion", env!("CARGO_PKG_VERSION"));
        res.set("InternalName", "Kireon Bootstrapper");
        res.set("OriginalFilename", "KireonPlayerLauncher.exe");
        res.set("CompanyName", "Brick Technologies, LLC");
        res.set("LegalCopyright", "Copyright (c) 2021");
        res.compile().unwrap();
    }

    println!("cargo:rerun-if-changed=build.rs");
}