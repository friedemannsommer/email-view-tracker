use std::{
    env, fs,
    path::{Path, PathBuf},
};

use lightningcss::stylesheet::{ParserOptions, PrinterOptions, StyleSheet};

const SHARED_STYLESHEET: &str = "shared.css";

fn main() {
    let out_dir = PathBuf::from(env::var_os("OUT_DIR").unwrap());
    let package_dir = PathBuf::from(env::var_os("CARGO_MANIFEST_DIR").unwrap());
    let asset_dir = package_dir.join("./src/assets");

    write_font_assets(&out_dir, &asset_dir);
    transform_stylesheets(&out_dir, &asset_dir);

    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed={}", asset_dir.to_str().unwrap());
}

fn generate_build_asset_file(path: &Path, contents: String) -> String {
    let metadata = path.metadata().unwrap();
    let last_modified = time::OffsetDateTime::from(metadata.modified().unwrap());

    format!(
        r#"crate::model::asset::Asset{{
        name: "{}",
        contents: &[{}],
        last_modified: time::macros::datetime!({} {} UTC)
    }}"#,
        path.file_name().unwrap().to_str().unwrap(),
        contents
            .as_bytes()
            .iter()
            .map(|b| b.to_string())
            .collect::<Vec<String>>()
            .join(","),
        last_modified.date(),
        last_modified.time()
    )
}

fn generate_static_asset_file(path: &Path) -> String {
    let metadata = path.metadata().unwrap();
    let last_modified = time::OffsetDateTime::from(metadata.modified().unwrap());

    format!(
        r#"crate::model::asset::Asset{{
        name: "{}",
        contents: include_bytes!("{}"),
        last_modified: time::macros::datetime!({} {} UTC)
    }}"#,
        path.file_name().unwrap().to_str().unwrap(),
        fs::canonicalize(path).unwrap().to_str().unwrap(),
        last_modified.date(),
        last_modified.time()
    )
}

fn write_font_assets(out_dir: &Path, asset_dir: &Path) {
    let font_dir = asset_dir.join("fonts/montserrat");

    for file_name in ["bold.woff", "bold.woff2", "regular.woff", "regular.woff2"] {
        let file = font_dir.join(file_name);

        fs::write(
            out_dir
                .join(file_name)
                .with_extension(std::fmt::format(format_args!(
                    "{}.rs",
                    file.extension().unwrap().to_str().unwrap()
                ))),
            generate_static_asset_file(&file),
        )
        .unwrap();
    }
}

fn transform_stylesheets(out_dir: &Path, asset_dir: &Path) {
    let shared_stylesheet =
        String::from_utf8(fs::read(asset_dir.join(SHARED_STYLESHEET)).unwrap()).unwrap();

    for dir_entry in fs::read_dir(asset_dir).unwrap().flatten() {
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
                    out_dir
                        .join(&filename)
                        .with_extension(std::fmt::format(format_args!(
                            "{}.rs",
                            extension.to_string_lossy()
                        ))),
                    generate_build_asset_file(
                        &path,
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
                    ),
                )
                .unwrap();
            }
        }
    }
}
