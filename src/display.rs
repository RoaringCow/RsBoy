use minifb::{Window, WindowOptions};

const WIDTH: usize = 640;
const HEIGHT: usize = 360;


pub struct Display {
    pub buffer: Vec<u32>,
    pub window: Window,
}

#[allow(dead_code)]
impl Display {
    // Create a new display
    pub fn new() -> Self {
        let mut window = Window::new(
            "Test - ESC to exit",
            WIDTH,
            HEIGHT,
            WindowOptions::default(),
        )
        .unwrap_or_else(|e| {
            panic!("{}", e);
        });

        window.limit_update_rate(Some(std::time::Duration::from_micros(166000)));

        Self {
            buffer: vec![0; HEIGHT * WIDTH], // Initialize buffer as needed
            window,
        }
    }

    pub fn update(&mut self) {
        self.window
            .update_with_buffer(&self.buffer, WIDTH, HEIGHT)
            .unwrap();
    }
}

