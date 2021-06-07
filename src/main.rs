mod space;
use space::*;

#[cfg(not(feature = "use_gtk"))]
pub fn main() {
    let mut sim: Simulator = space::Simulator::new(10);
    print!("{:#?}\n", sim);
    print!("{:#?}\n", sim.tree());
}

#[cfg(feature = "use_gtk")]
pub fn main() {
    use gio::prelude::*;
    use gtk::prelude::*;
    use gtk::*;

    let mut sim: Simulator = space::Simulator::new(100);
    sim.run();

    let application =
        Application::new(Some("com.github.gtk-rs.examples.basic"), Default::default())
            .expect("failed to initialize GTK application");

    application.connect_activate(move |app| {
        let window = ApplicationWindow::new(app);
        window.set_title("Space Sim");
        window.set_default_size(400, 400);

        let frame = gtk::Frame::new(None);
        let area = DrawingArea::new();
        // area.connect_draw(move |w, c| {
        //     println!("w: {} c:{}", w, c);
        //     c.rectangle(1.0, 1.0, 100.0, 200.0);
        //     for m in sim.masses {}
        //     c.fill();
        //     gtk::Inhibit(false)
        // });
        frame.add(&area);
        window.add(&frame);

        window.show_all();
    });

    application.run(&[]);
}
