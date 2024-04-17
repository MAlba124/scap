// use std::{
//     mem::size_of,
//     sync::{
//         atomic::{AtomicBool, AtomicU8},
//         mpsc::{self, sync_channel, SyncSender},
//     },
//     thread::JoinHandle,
//     time::Duration,
// };

// use xcb::x;

// use crate::{
//     capturer::Options,
//     frame::{BGRxFrame, Frame, RGBFrame, RGBxFrame, XBGRFrame},
// };

// static CAPTURER_STATE: AtomicU8 = AtomicU8::new(0);
// static STREAM_STATE_CHANGED_TO_ERROR: AtomicBool = AtomicBool::new(false);

// pub struct LinuxCapturer {
//     capturer_join_handle: Option<JoinHandle<()>>,
// }

// impl LinuxCapturer {
//     // TODO: Error handling
//     pub fn new(options: &Options, tx: mpsc::Sender<Frame>) -> Self {
//         // TODO: Fix this hack
//         let options = Options {
//             fps: options.fps,
//             show_cursor: options.show_cursor,
//             show_highlight: options.show_highlight,
//             output_type: options.output_type,
//             targets: options.targets.clone(),
//             excluded_targets: None,
//             output_resolution: crate::capturer::Resolution::Captured,
//             source_rect: None,
//         };
//         let (ready_sender, ready_recv) = sync_channel(1);
//         let capturer_join_handle = std::thread::spawn(move || {
//             let (conn, screen_num) = xcb::Connection::connect(None).unwrap();
//             let setup = conn.get_setup();
//             let screen = setup.roots().nth(screen_num as usize).unwrap();

//             let width = screen.width_in_pixels();
//             let height = screen.height_in_pixels();

//             ready_sender.send(true);

//             while CAPTURER_STATE.load(std::sync::atomic::Ordering::Relaxed) == 0 {
//                 std::thread::sleep(std::time::Duration::from_millis(10));
//             }

//             while CAPTURER_STATE.load(std::sync::atomic::Ordering::Relaxed) == 1 {
//                 let cookie = conn.send_request(&x::GetImage {
//                     format: x::ImageFormat::ZPixmap,
//                     drawable: x::Drawable::Window(screen.root()),
//                     x: 1920,
//                     y: 0,
//                     width: width - 1920,
//                     height,
//                     plane_mask: u32::MAX,
//                 });

//                 let reply = conn.wait_for_reply(cookie).unwrap();

//                 let src = reply.data();
//                 tx.send(
//                     Frame::BGR0(
//                         crate::frame::BGRFrame {
//                             display_time: 0,
//                             width: width as i32 - 1920,
//                             height: height as i32,
//                             data: src.to_vec(),
//                         }
//                     )
//                 ).unwrap();

//                 std::thread::sleep(std::time::Duration::from_millis(25));
//             }

//             // ready_sender.send(false)?;
//             ()
//         });

//         if !ready_recv.recv().expect("Failed to receive") {
//             panic!("Failed to setup capturer");
//         }

//         Self {
//             capturer_join_handle: Some(capturer_join_handle),
//         }
//     }

//     pub fn start_capture(&self) {
//         CAPTURER_STATE.store(1, std::sync::atomic::Ordering::Relaxed);
//     }

//     pub fn stop_capture(&mut self) {
//         CAPTURER_STATE.store(2, std::sync::atomic::Ordering::Relaxed);
//         if let Some(handle) = self.capturer_join_handle.take() {
//             handle.join().unwrap();
//             // if let Err(e) = handle.join().expect("Failed to join capturer thread") {
//             //     eprintln!("Error occured capturing: {e}");
//             // }
//         }
//     }
// }

// pub fn create_capturer(options: &Options, tx: mpsc::Sender<Frame>) -> LinuxCapturer {
//     LinuxCapturer::new(options, tx)
// }

use std::{
    mem::size_of,
    sync::{
        atomic::{AtomicBool, AtomicU8},
        mpsc::{self, sync_channel, SyncSender},
    },
    thread::JoinHandle,
    time::Duration,
};


use crate::{
    capturer::Options,
    frame::{BGRxFrame, Frame, RGBFrame, RGBxFrame, XBGRFrame},
};

static CAPTURER_STATE: AtomicU8 = AtomicU8::new(0);
static STREAM_STATE_CHANGED_TO_ERROR: AtomicBool = AtomicBool::new(false);



pub struct LinuxCapturer {}

impl LinuxCapturer {
    // TODO: Error handling
    pub fn new(options: &Options, tx: mpsc::Sender<Frame>) -> Self {
        Self {}
    }

    pub fn start_capture(&self) {
        CAPTURER_STATE.store(1, std::sync::atomic::Ordering::Relaxed);
    }

    pub fn stop_capture(&mut self) {
        CAPTURER_STATE.store(2, std::sync::atomic::Ordering::Relaxed);
    }
}

pub fn create_capturer(options: &Options, tx: mpsc::Sender<Frame>) -> LinuxCapturer {
    LinuxCapturer::new(options, tx)
}
