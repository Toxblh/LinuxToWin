extern crate gio;
extern crate gtk;

use gio::prelude::*;
use gtk::prelude::*;
use gtk::{ButtonsType, DialogFlags, MessageDialog, MessageType, Window};
use std::env;

fn main() {
    if gtk::init().is_err() {
        println!("Failed to initialize GTK.");
        return;
    }

    new_window();
    show_shutdown_modal();
}

fn new_window() {
    let app = gtk::Application::new(
        Some("com.toxblh.linux_to_win"),
        gio::ApplicationFlags::FLAGS_NONE,
    )
    .expect("Application::new failed");

    app.connect_activate(|app| {
        let window = gtk::ApplicationWindow::new(app);

        window.set_default_size(400, 400);
        window.set_title("Restart Menu");
        window.set_position(gtk::WindowPosition::Center);

        window.show_all();
    });

    app.run(&env::args().collect::<Vec<_>>());
}

fn show_shutdown_modal() {
    MessageDialog::new(
        None::<&Window>,
        DialogFlags::empty(),
        MessageType::Info,
        ButtonsType::OkCancel,
        "Shutdown!",
    )
    .run();
}
