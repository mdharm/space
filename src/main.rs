use space::joe::*;
use space::matt::*;
use space::*;

fn select_factory() -> Box<dyn SimFactory> {
    use std::env;

    let default_sim_factory: Box<dyn SimFactory> = Box::new(MattFactory {});
    let args: Vec<String> = env::args().collect();

    // validate number of arguments with default behavior
    if args.len() == 1 || args.len() > 2 {
        return default_sim_factory;
    }

    match args[1].parse::<i32>() {
        Ok(1) => Box::new(JoeFactory {}),
        Ok(2) => Box::new(MattFactory {}),
        Ok(_) | Err(_) => default_sim_factory,
    }
}

#[cfg(not(feature = "use_gtk"))]
pub fn main() {
    let factory = select_factory();
    let sim: Box<dyn Simulator> = factory.new(10);
    println!("{:#?}", sim);
}

#[cfg(feature = "use_gtk")]
pub fn main() {
    use gio::prelude::*;
    use gtk::prelude::*;
    use gtk::*;
    use std::cell::*;
    use std::sync::*;

    let factory = select_factory();
    let sim = Arc::new(RwLock::new(factory.new(5000)));
    let sim1 = sim.clone();
    std::thread::spawn(move || {
        std::thread::sleep(std::time::Duration::from_millis(1000));
        loop {
            if let Ok(ref mut s) = sim1.write() {
                s.step();
            }
            std::thread::sleep(std::time::Duration::from_millis(100));
        }
    });

    let application =
        Application::new(Some("com.github.gtk-rs.examples.basic"), Default::default())
            .expect("failed to initialize GTK application");
    application.connect_activate(move |app| {
        let window = ApplicationWindow::new(app);
        window.set_title(&("Space Sim: ".to_owned() + &factory.name()));

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
            if let Ok(s) = sim2.read() {
                let i: Vec<&Mass> = s.mass_iter().collect();
                let mut max = *max_max.borrow_mut();
                for m in i.iter() {
                    max = max.max(m.position.0.abs()).max(m.position.1.abs());
                    let x = (m.position.0 * width / max / 2.0) + (width / 2.0);
                    let y = (m.position.1 * height / max / 2.0) + (height / 2.0);
                    let size = m.mass * 10000.0;
                    cairo.rectangle(x, y, size, size);
                }
                max_max.replace(max);
                println!("draw max: {}", max);
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
