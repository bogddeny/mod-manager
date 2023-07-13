use adw::{
    prelude::*,
    Application,
    ActionRow,
    HeaderBar,
    Window
};
use gtk::{
    Box,
    ListBox,
    Orientation,
    SelectionMode
};

pub struct ModManagerWindow {
    window: Window
}

impl ModManagerWindow {
    pub fn new(application: &Application, title: &str) -> Self {
        let row = ActionRow::builder()
            .activatable(true)
            .title("Click me")
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