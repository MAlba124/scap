use screencapturekit::{
    sc_content_filter::{InitParams, SCContentFilter},
    sc_error_handler,
    sc_output_handler::{CMSampleBuffer, SCStreamOutputType, StreamOutput},
    sc_shareable_content::SCShareableContent,
    sc_stream::SCStream,
    sc_stream_configuration::SCStreamConfiguration,
    sc_sys::{CMSampleBufferGetFormatDescription, CMSampleBufferGetImageBuffer, SCFrameStatus},
};

use core_graphics::access::ScreenCaptureAccess;
use core_graphics::display::CGMainDisplayID;

use core_video_sys::{
    CVImageBufferRef, CVPixelBufferGetBaseAddress, CVPixelBufferGetBaseAddressOfPlane,
    CVPixelBufferGetBytesPerRow, CVPixelBufferGetHeight, CVPixelBufferGetPixelFormatType,
    CVPixelBufferGetWidth, CVPixelBufferLockBaseAddress, CVPixelBufferUnlockBaseAddress,
};

struct ConsoleErrorHandler;

impl sc_error_handler::StreamErrorHandler for ConsoleErrorHandler {
    fn on_error(&self) {
        println!("Error!");
    }
}

struct OutputHandler {}

impl StreamOutput for OutputHandler {
    fn did_output_sample_buffer(&self, sample: CMSampleBuffer, of_type: SCStreamOutputType) {
        match of_type {
            SCStreamOutputType::Screen => {
                let frame_status = sample.frame_status;

                match frame_status {
                    SCFrameStatus::Idle => {
                        return;
                    }
                    _ => {
                        let ptr = sample.ptr;
                        let timestamp = ptr.get_presentation_timestamp().value;
                        println!("Timestamp: {}", timestamp);

                        let attachments = ptr.get_attachments();

                        // println!("Sample ptr: {:?}", ptr.get_attachments());

                        // let pixel_buffer = sample_ptr.get_image_buffer() as CVImageBufferRef;

                        // unsafe {
                        //     CVPixelBufferLockBaseAddress(pixel_buffer, 0);

                        //     let base_address =
                        //         CVPixelBufferGetBaseAddressOfPlane(pixel_buffer, 0) as *const u8;
                        //     let pixel_format_type = CVPixelBufferGetPixelFormatType(pixel_buffer);

                        //     // get the pixel buffer's width and height
                        //     let width = CVPixelBufferGetWidth(pixel_buffer);
                        //     let height = CVPixelBufferGetHeight(pixel_buffer);

                        //     let bytes_per_row = CVPixelBufferGetBytesPerRow(pixel_buffer);

                        //     // Safe part starts here

                        //     CVPixelBufferUnlockBaseAddress(pixel_buffer, 0);
                        // }
                    }
                }
            }
            SCStreamOutputType::Audio => {
                // TODO: Handle audio
            }
        }
    }
}

pub fn main() {
    let content = SCShareableContent::current();
    let displays = content.displays;

    let main_display_id = unsafe { CGMainDisplayID() };
    let main_display = displays
        .iter()
        .find(|display| display.display_id == main_display_id)
        .unwrap_or_else(|| {
            panic!("Main display not found");
        });

    let width = main_display.width;
    let height = main_display.height;

    // Setup screencapturekit
    let params = InitParams::Display(main_display.clone());
    let filter = SCContentFilter::new(params);

    let stream_config = SCStreamConfiguration {
        shows_cursor: true,
        width,
        height,
        ..Default::default()
    };

    let error_handler = ConsoleErrorHandler;
    let mut stream = SCStream::new(filter, stream_config, error_handler);
    let output_handler = OutputHandler {};
    stream.add_output(output_handler);

    stream.start_capture();
    println!("Capture started. Press Enter to stop.");

    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();

    stream.stop_capture();
    println!("Capture stopped.");
}

pub fn has_permission() -> bool {
    let access = ScreenCaptureAccess::default();
    access.preflight()
}
