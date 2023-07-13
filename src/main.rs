mod app;
mod files;
mod win;

use gtk::{gio, glib};

use crate::app::ModManagerApplication;

const APPLICATION_ID: &str = "com.bogddeny.ModManager";
const APPLICATION_FLAGS: gio::ApplicationFlags = gio::ApplicationFlags::FLAGS_NONE;

fn main() -> glib::ExitCode {
    let application = ModManagerApplication::new(APPLICATION_ID, APPLICATION_FLAGS);

    application.run()
}
