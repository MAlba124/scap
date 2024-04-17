use std::sync::mpsc;

use crate::frame::Frame;
use crate::capturer::Options;

#[cfg(feature = "linux-wayland")]
mod wayland;

#[cfg(feature = "linux-x11")]
mod x11;

pub struct LinuxCapturer {
    #[cfg(feature = "linux-wayland")]
    inner: wayland::LinuxCapturer,
    #[cfg(feature = "linux-x11")]
    inner: x11::LinuxCapturer,
}

impl LinuxCapturer {
    pub fn new(options: &Options, tx: mpsc::Sender<Frame>) -> LinuxCapturer {
        #[cfg(not(any(feature = "linux-wayland", feature = "linux-x11")))]
        panic!("No linux feature enabled");

        Self {
            #[cfg(feature = "linux-wayland")]
            inner: wayland::LinuxCapturer::new(options, tx),
            #[cfg(feature = "linux-x11")]
            inner: x11::LinuxCapturer::new(options, tx),
        }
    }

    pub fn start_capture(&self) {
        #[cfg(any(feature = "linux-wayland", feature = "linux-x11"))]
        {
            self.inner.start_capture();
            return;
        }

        panic!("No linux feature enabled");
    }

    pub fn stop_capture(&mut self) {
        #[cfg(any(feature = "linux-wayland", feature = "linux-x11"))]
        {
            self.inner.stop_capture();
            return;
        }

        panic!("No linux feature enabled");
    }
}


pub fn create_capturer(options: &Options, tx: mpsc::Sender<Frame>) -> LinuxCapturer {
    LinuxCapturer::new(options, tx)
}
