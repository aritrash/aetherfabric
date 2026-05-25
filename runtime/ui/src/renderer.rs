// runtime/ui/src/renderer.rs

use crate::dashboard::DashboardState;

use eframe::{egui, App, CreationContext, Frame};

/// Primary AetherFabric UI renderer.
///
/// Responsible for:
/// - kiosk rendering
/// - framebuffer presentation
/// - UI lifecycle management
/// - runtime visualization
///
/// IMPORTANT:
/// This renderer DOES NOT:
/// - own orchestration logic
/// - schedule workloads
/// - manage runtime state
///
/// It only visualizes orchestration state.
pub struct AetherRenderer {
    /// Dashboard UI state
    dashboard: DashboardState,
}

impl AetherRenderer {
    /// Creates renderer instance.
    pub fn new(_cc: &CreationContext<'_>) -> Self {
        Self {
            dashboard: DashboardState::new(),
        }
    }
}

impl App for AetherRenderer {
    /// Main render/update loop.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut Frame) {
        // -------------------------------------------------
        // Visual Theme Configuration
        // -------------------------------------------------

        configure_theme(ctx);

        // -------------------------------------------------
        // Dashboard Rendering
        // -------------------------------------------------

        self.dashboard.render(ctx);

        // -------------------------------------------------
        // Frame Scheduling
        // -------------------------------------------------

        // Keeps UI continuously refreshing.
        ctx.request_repaint();
    }
}

/// Configures global UI theme.
///
/// This defines:
/// - dark appliance aesthetic
/// - neon accent visibility
/// - kiosk readability
/// - touch-friendly spacing
fn configure_theme(ctx: &egui::Context) {
    let mut style = (*ctx.style()).clone();

    // -------------------------------------------------
    // Global Visuals
    // -------------------------------------------------

    style.visuals = egui::Visuals::dark();

    style.visuals.panel_fill = egui::Color32::from_rgb(4, 8, 16);

    style.visuals.window_fill = egui::Color32::from_rgb(6, 10, 18);

    style.visuals.extreme_bg_color = egui::Color32::from_rgb(2, 4, 10);

    style.visuals.widgets.active.bg_fill = egui::Color32::from_rgb(0, 120, 255);

    style.visuals.widgets.hovered.bg_fill = egui::Color32::from_rgb(0, 90, 180);

    style.visuals.widgets.inactive.bg_fill = egui::Color32::from_rgb(10, 14, 24);

    // -------------------------------------------------
    // Text Styling
    // -------------------------------------------------

    style.visuals.override_text_color = Some(egui::Color32::WHITE);

    // -------------------------------------------------
    // Spacing
    // -------------------------------------------------

    style.spacing.item_spacing = egui::vec2(10.0, 10.0);

    style.spacing.button_padding = egui::vec2(14.0, 14.0);

    style.spacing.window_margin = egui::Margin::same(14.0);

    // -------------------------------------------------
    // Rounded Appliance Aesthetic
    // -------------------------------------------------

    style.visuals.widgets.active.rounding = egui::Rounding::same(14.0);

    style.visuals.widgets.hovered.rounding = egui::Rounding::same(14.0);

    style.visuals.widgets.inactive.rounding = egui::Rounding::same(14.0);

    // -------------------------------------------------
    // Touchscreen Optimizations
    // -------------------------------------------------

    style.interaction.resize_grab_radius_side = 12.0;

    style.interaction.resize_grab_radius_corner = 16.0;

    // -------------------------------------------------
    // Apply Theme
    // -------------------------------------------------

    ctx.set_style(style);
}
