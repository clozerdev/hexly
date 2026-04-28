#[rustfmt::skip]
mod config;

mod app;
mod core;
mod ui;
mod utils;

use gtk::prelude::*;
use gtk::{gio, glib};

use self::app::HexlyApplication;
use self::config::RESOURCES_FILE;

fn main() -> glib::ExitCode {
    let res = gio::Resource::load(RESOURCES_FILE).expect("Could not load gresource file");
    gio::resources_register(&res);

    let app = HexlyApplication::new();
    app.run()
}
