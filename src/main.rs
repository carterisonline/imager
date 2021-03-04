#![feature(with_options)]
use gio::prelude::*;
use gtk::prelude::*;

use std::env::args;
mod imagefile;

fn build_ui(application: &gtk::Application) {
    // Configure button as drag source for text
    let targets = vec![gtk::TargetEntry::new(
        "text/uri-list",
        gtk::TargetFlags::OTHER_APP,
        0,
    )];

    // Configure to_image as drag destination to receive text
    let to_image = gtk::Label::new(Some(
        "Drop file here, and it will be converted into an image.\n",
    ));
    to_image.drag_dest_set(gtk::DestDefaults::ALL, &targets, gdk::DragAction::COPY);
    to_image.connect_drag_data_received(|w, _, _, _, d, _, _| {
        w.set_text(
            "Drop file here, and it will be converted into an image.\n✅ Successfully converted file!",
        );
        for file in d.get_uris() {
            let file = gio::File::new_for_uri(&file);
            let display_name = if file.is_native() {
                file.get_path().unwrap().display().to_string()
            } else {
                file.get_uri().into()
            };
            println!("{}", display_name);
            imagefile::save_file(&display_name);
        }
    });

    let from_image = gtk::Label::new(Some("OR, drop an image here to decrypt it.\n"));
    from_image.drag_dest_set(gtk::DestDefaults::ALL, &targets, gdk::DragAction::COPY);
    from_image.connect_drag_data_received(|w, _, _, _, d, _, _| {
        w.set_text("OR, drop an image here to decrypt it.\n✅ Successfully converted file!");
        for file in d.get_uris() {
            let file = gio::File::new_for_uri(&file);
            let display_name = if file.is_native() {
                file.get_path().unwrap().display().to_string()
            } else {
                file.get_uri().into()
            };
            println!("{}", display_name);
            imagefile::from_image(&display_name);
        }
    });

    let separator = gtk::Separator::new(gtk::Orientation::Vertical);

    // Stack the button and to_image horizontally
    let hbox = gtk::Box::new(gtk::Orientation::Horizontal, 0);
    hbox.pack_start(&to_image, true, true, 0);
    hbox.pack_start(&separator, true, false, 0);
    hbox.pack_start(&from_image, true, true, 0);

    // Finish populating the window and display everything
    let window = gtk::ApplicationWindow::new(application);
    window.set_title("Imager");
    window.set_default_size(800, 300);
    window.add(&hbox);
    window.show_all();
}

fn main() {
    let application = gtk::Application::new(Some("com.carterisonline.imager"), Default::default())
        .expect("Initialization failed...");

    application.connect_activate(|app| {
        build_ui(app);
    });

    application.run(&args().collect::<Vec<_>>());
}
