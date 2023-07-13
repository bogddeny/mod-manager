use adw::{
    prelude::*,
    Application,
    ActionRow,
    HeaderBar,
    Window
};
use gtk::{
    Box,
    Button,
    ListBox,
    Orientation,
    SelectionMode
};

use crate::files::{self, read_directory_folders};

pub struct ModManagerWindow {
    window: Window
}

impl ModManagerWindow {
    pub fn new(application: &Application, title: &str) -> Self {
        let button: Button = Button::builder()
            .label("Select Mod Path")
            .build();

        let mod_list_box = ListBox::builder()
            .margin_top(16)
            .margin_bottom(16)
            .margin_start(16)
            .margin_end(16)
            .selection_mode(SelectionMode::None)
            .css_classes(vec![String::from("boxed-list")])
            .build();
        
        //let folders = read_directory_folders("path_to_directory");
//
        //for folder in folders {
        //    let mod_name = Some(&folder.file_name().unwrap().to_string_lossy());
        //    let mod_version = 
        //}



        let row = ActionRow::builder()
            .activatable(true)
            .title("Click me")
            .subtitle("")
            .build();
        row.connect_activated(|_| {
            eprintln!("Clicked!");
        });

        let list = ListBox::builder()
            .margin_top(32)
            .margin_end(32)
            .margin_bottom(32)
            .margin_start(32)
            .selection_mode(SelectionMode::None)
            // makes the list look nicer
            .css_classes(vec![String::from("boxed-list")])
            .build();
        list.append(&row);

        // Combine the content in a box
        let content = Box::new(Orientation::Vertical, 0);
        // Adwaitas' ApplicationWindow does not include a HeaderBar
        content.append(&HeaderBar::new());
        content.append(&button);
        content.append(&list);


        let window = Window::builder()
            .application(application)
            .default_width(1600)
            .default_height(900)
            .title(title)
            .content(&content)
            .build();

        ModManagerWindow { window }
    }
    pub fn present(&self) {
        self.window.present()
    }
}