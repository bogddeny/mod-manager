use adw::{
    prelude::*,
    Application,
    ActionRow,
    traits::ActionRowExt,
    HeaderBar,
    Window
};
use gtk::{
    Box,
    Button,
    CheckButton,
    Label,
    ListBox,
    Orientation,
    SelectionMode,
};

use crate::files::*;

pub struct ModManagerWindow {
    window: Window
}

impl ModManagerWindow {
    pub fn new(application: &Application, title: &str) -> Self {
        let button: Button = Button::builder()
            .label("Select Mod Path")
            .margin_top(16)
            .margin_bottom(16)
            .margin_start(16)
            .margin_end(16)
            .build();

        let mod_list_box = ListBox::builder()
            .margin_top(16)
            .margin_bottom(16)
            .margin_start(16)
            .margin_end(16)
            .selection_mode(SelectionMode::None)
            .css_classes(vec![String::from("boxed-list")])
            .build();
        
        let directory = "/home/bogdan/Documents/Projects/mod-manager/mods";

        let folders = read_directory_folders(directory);

        for (i, folder) in folders.iter().enumerate() {
            let mod_name: String = folder.file_name().unwrap().to_string_lossy().into();
            let mod_version: String = read_meta_properties(&folder).unwrap();

            let check_button = CheckButton::new();
            check_button.set_margin_end(16);
            let mod_action_row = ActionRow::builder()
                .activatable(true)
                .title(mod_name)
                .subtitle(mod_version)
                .activatable_widget(&check_button)
                .build();
            let label = Label::new(Some(i.to_string().as_str()));
            mod_action_row.add_prefix(&check_button);
            mod_action_row.add_suffix(&label);

            mod_list_box.append(&mod_action_row);
        }

        let content = Box::new(Orientation::Vertical, 0);
        content.append(&HeaderBar::new());
        content.append(&button);
        content.append(&mod_list_box);


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