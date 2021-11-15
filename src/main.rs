#![forbid(unsafe_code)]
#![cfg_attr(not(debug_assertions), deny(warnings))] // Forbid warnings in release builds
#![warn(clippy::all, rust_2018_idioms)]

// When compiling natively:
#[cfg(not(target_arch = "wasm32"))]
fn main() {
    // use windows::{ApplicationModel::DataTransfer::Clipboard, Foundation::EventHandler};

    // let event = EventHandler::new(|_a, _b| {
    //     println!("xd");

    //     Ok(())
    // });

    // Clipboard::ContentChanged(event).unwrap();

    let app = pastabox::PastaBox::default();
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(Box::new(app), native_options);
}
