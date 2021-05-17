extern crate gio;
extern crate gtk;

use gio::prelude::*;
use gtk::prelude::*;
use gtk::*;
use std::error::Error;

mod space;

pub fn main() {
    run();
}

pub fn run() -> Result<(), std::boxed::Box<dyn Error>> {
    let application =
        Application::new(Some("com.github.gtk-rs.examples.basic"), Default::default())
            .expect("failed to initialize GTK application");

    application.connect_activate(move |app| {
        let window = ApplicationWindow::new(app);
        window.set_title("Space Sim");
        window.set_default_size(400, 400);

        let frame = gtk::Frame::new(None);
        let area = DrawingArea::new();
        area.connect_draw(move |w, c| {
            println!("w: {} c:{}", w, c);
            c.rectangle(1.0, 1.0, 100.0, 200.0);
            c.fill();
            gtk::Inhibit(false)
        });
        frame.add(&area);
        window.add(&frame);

        window.show_all();
    });

    application.run(&[]);
    return Ok(());
}
