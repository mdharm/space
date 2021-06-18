use space::*;
use std::sync::*;

#[cfg(not(feature = "use_gtk"))]
pub fn main() {
    let mut sim: Simulator = Simulator::new(10);
    print!("{:#?}", sim);
    print!("{:#?}", sim.tree());
}

#[cfg(feature = "use_gtk")]
pub fn main() {
    use gio::prelude::*;
    use gtk::prelude::*;
    use gtk::*;

    let sim = Arc::new(RwLock::new(Simulator::new(500)));
    let sim1 = sim.clone();
    std::thread::spawn(move || loop {
        let s = sim1.read().unwrap().step();
        sim1.write().unwrap().tree = s;
        //std::thread::sleep_ms(200);
    });

    let application =
        Application::new(Some("com.github.gtk-rs.examples.basic"), Default::default())
            .expect("failed to initialize GTK application");
    application.connect_activate(move |app| {
        let window = ApplicationWindow::new(app);
        window.set_title("Space Sim");

        const WIDTH: f64 = 400.0;
        const HEIGHT: f64 = 400.0;
        window.set_default_size(WIDTH as i32, HEIGHT as i32);

        let frame = gtk::Frame::new(None);
        let area = DrawingArea::new();

        let sim2 = sim.clone();
        area.connect_draw(move |_w, cairo| {
            println!("draw");
            let s = sim2.read().unwrap();
            let i: Vec<&Mass> = s.tree.mass_iter().collect();
            for m in i.iter() {
                let x = WIDTH / 2.0 + 100.0 * m.position.x;
                let y = HEIGHT / 2.0 + 100.0 * m.position.y;
                println!(
                    "draw ({}, {}) -> ({}, {})",
                    m.position.x, m.position.y, x, y
                );
                cairo.rectangle(x, y, 1.0, 1.0);
            }
            cairo.fill();
            gtk::Inhibit(false)
        });
        frame.add(&area);
        window.add(&frame);
        window.show_all();

        glib::source::timeout_add_local(50, move || {
            area.queue_draw();
            Continue(true)
        });
    });

    application.run(&[]);
}
