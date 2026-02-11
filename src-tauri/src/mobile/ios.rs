use objc::runtime::{Object, BOOL};
use objc::{class, msg_send, sel, sel_impl};
use std::ffi::CString;

// Link AVFoundation so AVAudioSession is available at runtime.
#[link(name = "AVFoundation", kind = "framework")]
extern "C" {}

/// Configure AVAudioSession with the `.playback` category.
///
/// This tells iOS the app plays audio and should keep running in the
/// background while audio is active. When playback stops the system
/// will suspend the app normally.
fn configure_audio_session() {
    unsafe {
        let session: *mut Object = msg_send![class!(AVAudioSession), sharedInstance];
        if session.is_null() {
            eprintln!("[Monochrome] AVAudioSession shared instance not available");
            return;
        }

        let category_cstr = match CString::new("AVAudioSessionCategoryPlayback") {
            Ok(value) => value,
            Err(_) => {
                eprintln!("[Monochrome] Failed to build AVAudioSession category string");
                return;
            }
        };

        let category: *mut Object =
            msg_send![class!(NSString), stringWithUTF8String: category_cstr.as_ptr()];
        if category.is_null() {
            eprintln!("[Monochrome] Failed to build AVAudioSession category string");
            return;
        }

        let mut error: *mut Object = std::ptr::null_mut();

        let _: BOOL = msg_send![session, setCategory: category error: &mut error];
        let _: BOOL = msg_send![session, setActive: true error: &mut error];

        println!("[Monochrome] AVAudioSession configured for background playback");
    }
}

pub fn setup(_app: &mut tauri::App) -> Result<(), Box<dyn std::error::Error>> {
    configure_audio_session();
    Ok(())
}
