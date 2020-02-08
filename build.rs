use clap::{Shell};
use std::fs;

include!("src/cli.rs");

fn main() {
    // https://doc.rust-lang.org/cargo/reference/build-scripts.html#outputs-of-the-build-script
    let outdir = std::env::var_os("OUT_DIR").expect("failed to find OUT_DIR");
    fs::create_dir_all(&outdir).expect("failed to create dirs for OUT_DIR");

    // Generate shell completions.
    let mut app = build_app();
    let app_name = crate_name!();
    app.gen_completions(app_name, Shell::Bash, &outdir);
    app.gen_completions(app_name, Shell::Fish, &outdir);
    app.gen_completions(app_name, Shell::Zsh, &outdir);
    app.gen_completions(app_name, Shell::PowerShell, &outdir);
}
