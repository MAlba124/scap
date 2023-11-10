use std::{path::PathBuf, time::Instant};
use windows::Win32::Graphics::Gdi::{GetMonitorInfoW, HMONITOR, MONITORINFOEXW};
use windows_capture::{
    capture::WindowsCaptureHandler, frame::Frame, graphics_capture_api::GraphicsCaptureApi,
    monitor::Monitor, settings::WindowsCaptureSettings, window::Window,
};

use crate::audio;
use crate::{Target, TargetKind};

struct Recorder {
    frames: usize,
}

// IMPROVE: get user-friendly monitor name
fn get_monitor_name(h_monitor: HMONITOR) -> windows::core::Result<String> {
    let mut monitor_info = MONITORINFOEXW::default();
    monitor_info.monitorInfo.cbSize = std::mem::size_of::<MONITORINFOEXW>() as u32;

    let success =
        unsafe { GetMonitorInfoW(h_monitor, &mut monitor_info as *mut _ as *mut _).as_bool() };

    if success {
        let len = monitor_info
            .szDevice
            .iter()
            .position(|&i| i == 0)
            .unwrap_or(0);
        let name = String::from_utf16(&monitor_info.szDevice[..len]).unwrap();

        let clean_name = match name.rfind('\\') {
            Some(index) => name.chars().skip(index + 1).collect(),
            None => name.to_string(),
        };

        Ok(clean_name)
    } else {
        Err(windows::core::Error::new(
            windows::core::HRESULT(0),
            "Failed to get monitor info".into(),
        ))
    }
}

impl WindowsCaptureHandler for Recorder {
    type Flags = ();

    fn new(_: Self::Flags) -> Self {
        Self { frames: 0 }
    }

    fn on_frame_arrived(&mut self, frame: Frame) {
        self.frames += 1;

        println!("frame: {}", self.frames);
        println!("size: {}x{}", frame.width(), frame.height());

        // println!("buffer: {:?}", frame.buffer());

        let filename = format!("./test/frame-{}.png", self.frames);
        println!("filename: {}", filename);

        frame.save_as_image(&filename).unwrap();

        // TODO: encode the frames received here into a video
    }

    fn on_closed(&mut self) {
        println!("Closed");
    }
}

fn remove_null_character(input: &str) -> String {
    match input.strip_suffix('\0') {
        Some(s) => s.to_string(),
        None => input.to_string(),
    }
}

pub fn main() {
    let settings =
        WindowsCaptureSettings::new(Monitor::primary(), Some(true), Some(false), ()).unwrap();

    println!("Capture started. Press Enter to stop.");

    let mut audio_recorder = audio::AudioRecorder::new();

    audio_recorder.start_recording();

    Recorder::start(settings).unwrap();

    // audio_recorder.stop_recording();

    // TODO: figure out threading mechanism here
}

pub fn is_supported() -> bool {
    GraphicsCaptureApi::is_supported().expect("Failed to check support")
}

pub fn has_permission() -> bool {
    // TODO: add correct permission mechanism here
    true
}

pub fn get_targets() -> Vec<Target> {
    let mut targets: Vec<Target> = Vec::new();

    let displays = Monitor::enumerate().expect("Failed to enumerate monitors");

    for display in displays {
        let id = display;
        let name = get_monitor_name(display).unwrap();

        let target = Target {
            id: 2,
            title: name,
            kind: TargetKind::Display,
        };
        targets.push(target);
    }

    let windows = Window::enumerate().expect("Failed to enumerate windows");
    for window in windows {
        let handle = window.as_raw_hwnd();

        let title = window
            .title()
            .unwrap()
            .strip_suffix('\0')
            .unwrap()
            .to_string();

        let target = Target {
            id: 3,
            kind: TargetKind::Window,
            title,
        };
        targets.push(target);
    }

    targets
}