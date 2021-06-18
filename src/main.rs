use space::*;

#[cfg(not(feature = "use_gtk"))]
pub fn main() {
    let sim: Simulator = Simulator::new(10);
    print!("{:#?}", sim);
    print!("{:#?}", sim.tree);
}

#[cfg(feature = "use_gtk")]
pub fn main() {
    use gio::prelude::*;
    use gtk::prelude::*;
    use gtk::*;
    use std::cell::*;
    use std::sync::*;

    let sim = Arc::new(RwLock::new(Simulator::new(5000)));
    let sim1 = sim.clone();
    std::thread::spawn(move || loop {
        let new_tree = sim1.read().unwrap().new_tree();
        sim1.write().unwrap().tree = new_tree;
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
        let max_max: RefCell<f64> = RefCell::new(0.0);
        area.connect_draw(move |window, cairo| {
            let width = window.get_allocated_width() as f64;
            let height = window.get_allocated_height() as f64;
            let s = sim2.read().unwrap();
            let i: Vec<&Mass> = s.tree.mass_iter().collect();
            let mut max = *max_max.borrow_mut();
            for m in i.iter() {
                max = max.max(m.position.x.abs()).max(m.position.y.abs());
                let x = (m.position.x / max + 0.5) * width;
                let y = (m.position.y / max + 0.5) * height;
                let size = m.mass * 10000.0;
                cairo.rectangle(x, y, size, size);
            }
            max_max.replace(max);
            println!("draw max: {}", max);
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
