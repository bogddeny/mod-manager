mod files;

use files::read_directory_folders;
use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, CheckButton, Grid, Orientation, Label};
use std::fs::File;
use std::io::{BufRead, BufReader};

use crate::files::read_meta_properties;

fn main() {
    let application = Application::new(
        Some("com.example.folder_app"),
        Default::default(),
    );

    application.connect_activate(|app| {
        let window = ApplicationWindow::new(app);
        window.set_title(Some("Folder Application"));
        window.set_default_size(400, 300);

        let grid = Grid::new();
        grid.set_orientation(Orientation::Vertical);
        window.set_child(Some(&grid));

        let directory = "/home/bogdan/Documents/Projects/mod-manager/mods";
        let folders = read_directory_folders(directory);

        for (index, folder) in folders.iter().enumerate() {
            let check_button = CheckButton::new();
            let label = Label::new(Some(&folder.file_name().unwrap().to_string_lossy()));
            let version_label = Label::new(None);

            let version = read_meta_properties(folder);
            version_label.set_text(version.as_deref().unwrap_or_default());

            grid.attach(&check_button, 0, index as i32, 1, 1);
            grid.attach(&label, 1, index as i32, 1, 1);
            grid.attach(&version_label, 2, index as i32, 1, 1);
        }

        window.present();
    });

    application.run();
}
