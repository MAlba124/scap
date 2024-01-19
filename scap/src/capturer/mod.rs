mod engine;

use std::sync::mpsc;

use crate::{device::display, frame::{Frame, FrameType}};

#[derive(Debug, Default, Clone)]
pub struct CGPoint {
    pub x: f64,
    pub y: f64,
}

#[derive(Debug, Default, Clone)]
pub struct CGSize {
    pub width: f64,
    pub height: f64,
}
#[derive(Debug, Default, Clone)]
pub struct CGRect {
    pub origin: CGPoint,
    pub size: CGSize,
}
#[derive(Debug, Default)]
pub struct Options {
    pub fps: u32,
    pub show_cursor: bool,
    pub show_highlight: bool,
    pub targets: Vec<display::Target>,

    // excluded targets will only work on macOS
    pub excluded_targets: Option<Vec<display::Target>>,
    pub output_type: FrameType,
    pub source_rect: Option<CGRect>
}

pub struct Capturer {
    engine: engine::Engine,
    rx: mpsc::Receiver<Frame>,
}

impl Capturer {
    pub fn new(options: Options) -> Capturer {
        let (tx, rx) = mpsc::channel::<Frame>();
        let engine = engine::Engine::new(&options, tx);

        Capturer { engine, rx }
    }

    // TODO
    // Prevent starting capture if already started
    pub fn start_capture(&self) {

        self.engine.start();
    }

    pub fn stop_capture(&self) {
        self.engine.stop();
    }

    pub fn get_next_frame(&self) -> Result<Frame, mpsc::RecvError> {
        self.rx.recv()
    }
}