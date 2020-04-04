use clap::{
    app_from_crate, crate_authors, crate_description, crate_name, crate_version, App, AppSettings,
};

pub fn build_app() -> App<'static, 'static> {
    app_from_crate!().setting(AppSettings::ColoredHelp)
}
