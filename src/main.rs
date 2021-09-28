use gtk4 as gtk;
use gtk::prelude::*;
use gtk::{Application, ApplicationWindow};

mod model;
mod tree_view;

use tree_view::tree;

fn main() {
    let app = Application::builder()
        .application_id("org.example.TreeViewTest")
        .build();

    app.connect_activate(| app| {
        // We create the main window.
        let window = ApplicationWindow::builder()
            .application(app)
            .default_width(320)
            .default_height(200)
            .title("TreeViewTest")
            .build();

        let tree = tree();
        window.set_child(Some(&tree));

        // Show the window.
        window.show();
    });

    app.run();
}