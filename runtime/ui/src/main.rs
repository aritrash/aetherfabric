// runtime/ui/src/main.rs

mod dashboard;
mod renderer;
mod state;
mod touch;
mod widgets;

use renderer::AetherRenderer;

use eframe::{egui, NativeOptions};

fn main() -> Result<(), eframe::Error> {
    // -------------------------------------------------
    // Logging Initialization
    // -------------------------------------------------

    tracing_subscriber::fmt::init();

    // -------------------------------------------------
    // Native Window Configuration
    // -------------------------------------------------

    let mut options = NativeOptions::default();

    // -------------------------------------------------
    // Kiosk / Embedded Appliance Mode
    // -------------------------------------------------

    options.viewport = egui::ViewportBuilder::default()
        .with_inner_size([1280.0, 720.0])
        .with_min_inner_size([800.0, 480.0])
        .with_fullscreen(true)
        .with_decorations(false)
        .with_resizable(false)
        .with_drag_and_drop(false)
        .with_title("AetherFabric Runtime");

    // -------------------------------------------------
    // Launch Runtime UI
    // -------------------------------------------------

    eframe::run_native(
        "AetherFabric Runtime",
        options,
        Box::new(|cc| Box::new(AetherRenderer::new(cc))),
    )
}
