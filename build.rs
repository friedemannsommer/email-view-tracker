use std::{env, fs, path::PathBuf};

use lightningcss::stylesheet::{ParserOptions, PrinterOptions, StyleSheet};

const SHARED_STYLESHEET: &str = "shared.css";

fn main() {
    let out_dir = PathBuf::from(env::var_os("OUT_DIR").unwrap());
    let package_dir = PathBuf::from(env::var_os("CARGO_MANIFEST_DIR").unwrap());
    let asset_dir = package_dir.join("./src/assets");
    let shared_stylesheet =
        String::from_utf8(fs::read(asset_dir.join("shared.css")).unwrap()).unwrap();

    for dir_entry in fs::read_dir(&asset_dir).unwrap().flatten() {
        let path = dir_entry.path();
        let filename = dir_entry.file_name();

        if let Some(extension) = path.extension() {
            if extension == "css" && filename != SHARED_STYLESHEET {
                let file_contents = format!(
                    "{}\n{}",
                    shared_stylesheet,
                    String::from_utf8(fs::read(&path).unwrap()).unwrap()
                );

                fs::write(
                    out_dir.join(&filename),
                    StyleSheet::parse(
                        file_contents.as_str(),
                        ParserOptions {
                            filename: filename.to_str().unwrap().to_string(),
                            ..ParserOptions::default()
                        },
                    )
                    .unwrap()
                    .to_css(PrinterOptions {
                        minify: true,
                        ..Default::default()
                    })
                    .unwrap()
                    .code,
                )
                .unwrap();
            }
        }
    }

    println!("cargo:rerun-if-changed=build.rs");
    println!(
        "cargo:rerun-if-changed={}",
        asset_dir.as_os_str().to_str().unwrap()
    );
}
