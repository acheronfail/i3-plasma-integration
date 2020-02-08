use clap::{App, AppSettings, app_from_crate, crate_name, crate_authors, crate_version, crate_description};

pub fn build_app() -> App<'static, 'static> {
    app_from_crate!().setting(AppSettings::ColoredHelp)
}
