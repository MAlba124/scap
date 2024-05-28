use super::{Display, Target};
use windows::Win32::Graphics::Gdi::{GetMonitorInfoW, MONITORINFOEXW};
use windows_capture::{monitor::Monitor, window::Window};

pub use windows::Win32::{Foundation::HWND, Graphics::Gdi::HMONITOR};

use windows::Win32::Graphics::Gdi::{GetDC, GetDeviceCaps, ReleaseDC, LOGPIXELSX, LOGPIXELSY};

fn get_monitor_name(h_monitor: HMONITOR) -> String {
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

        clean_name
    } else {
        format!("Unknown Monitor {}", h_monitor.0)
    }
}

pub fn get_targets() -> Vec<Target> {
    let mut targets: Vec<Target> = Vec::new();

    let displays = Monitor::enumerate().expect("Failed to enumerate monitors");

    for display in displays {
        let id = display.as_raw_hmonitor() as u32;
        let title = get_monitor_name(HMONITOR(id as isize));

        let target = Target::Display(super::Display {
            id,
            title,
            raw_handle: HMONITOR(display.as_raw_hmonitor()),
        });
        targets.push(target);
    }

    let windows = Window::enumerate().expect("Failed to enumerate windows");
    for window in windows {
        let handle = window.as_raw_hwnd();

        let title = window.title().unwrap().to_string();

        let target = Target::Window(super::Window {
            id: 3,
            title,
            raw_handle: HWND(handle),
        });
        targets.push(target);
    }

    targets
}

pub fn get_main_display() -> Display {
    let display = Monitor::primary().expect("Failed to get primary monitor");
    let id = display.as_raw_hmonitor() as u32;

    Display {
        id,
        title: get_monitor_name(HMONITOR(display.as_raw_hmonitor())),
        raw_handle: HMONITOR(display.as_raw_hmonitor()),
    }
}

pub fn get_scale_factor() -> f64 {
    unsafe {
        let hdc = GetDC(None);

        let dpi_x = GetDeviceCaps(hdc, LOGPIXELSX);
        let dpi_y = GetDeviceCaps(hdc, LOGPIXELSY);

        ReleaseDC(None, hdc);

        let scale_x = dpi_x as f64 / 96.0;
        let scale_y = dpi_y as f64 / 96.0;
        let scale = (scale_x + scale_y) / 2.0;

        return scale;
    }
}