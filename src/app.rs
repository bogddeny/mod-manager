use adw::{
    prelude::*,
    Application
};
use gtk::{gio, glib};

use crate::win::ModManagerWindow;

const APPLICATION_NAME: &str = "Mod Manager NG";

pub struct ModManagerApplication {
    application: Application,
}

impl ModManagerApplication {
    pub fn new(application_id: &str, flags: gio::ApplicationFlags) -> Self {
        let application = Application::builder()
            .application_id(application_id)
            .flags(flags)
            .build();

        application.connect_activate(|app| {
            let window = ModManagerWindow::new(app, APPLICATION_NAME);

            window.present();
        });

        ModManagerApplication { application }
    }

    pub fn run(&self) -> glib::ExitCode {
        self.application.run()
    }
}
