// runtime/ui/src/widgets.rs

use egui::{Color32, Frame, Margin, ProgressBar, RichText, Rounding, Stroke, Ui, Vec2};

/// Generic neon-styled metric card.
///
/// Used throughout:
/// - Pi telemetry
/// - ESP telemetry
/// - scheduler statistics
/// - cooling observability
pub fn metric_card(ui: &mut Ui, title: &str, value: &str, accent: Color32, icon: &str) {
    Frame::none()
        .fill(Color32::from_rgb(8, 12, 22))
        .stroke(Stroke::new(1.2, accent))
        .rounding(Rounding::same(16.0))
        .inner_margin(Margin::same(14.0))
        .show(ui, |ui| {
            ui.set_min_size(Vec2::new(180.0, 120.0));

            ui.vertical_centered(|ui| {
                ui.label(RichText::new(icon).size(28.0).color(accent));

                ui.add_space(12.0);

                ui.label(RichText::new(title).size(18.0).color(Color32::WHITE));

                ui.add_space(8.0);

                ui.label(RichText::new(value).size(28.0).strong().color(accent));
            });
        });
}

/// Neon-styled progress panel.
///
/// Used for:
/// - CPU usage
/// - RAM usage
/// - ESP utilization
/// - workload distribution
pub fn progress_panel(ui: &mut Ui, title: &str, progress: f32, accent: Color32, icon: &str) {
    Frame::none()
        .fill(Color32::from_rgb(8, 12, 22))
        .stroke(Stroke::new(1.2, accent))
        .rounding(Rounding::same(16.0))
        .inner_margin(Margin::same(14.0))
        .show(ui, |ui| {
            ui.set_min_size(Vec2::new(220.0, 140.0));

            ui.vertical_centered(|ui| {
                ui.label(RichText::new(icon).size(30.0).color(accent));

                ui.add_space(10.0);

                ui.label(RichText::new(title).size(18.0).color(Color32::WHITE));

                ui.add_space(16.0);

                ui.add(ProgressBar::new(progress).desired_width(160.0).fill(accent));

                ui.add_space(10.0);

                ui.label(
                    RichText::new(format!("{:.0}%", progress * 100.0))
                        .size(24.0)
                        .strong()
                        .color(accent),
                );
            });
        });
}

/// Compact status indicator widget.
///
/// Useful for:
/// - online state
/// - node health
/// - orchestration visibility
pub fn status_dot(ui: &mut Ui, online: bool) {
    let color = if online {
        Color32::from_rgb(0, 255, 120)
    } else {
        Color32::from_rgb(255, 60, 60)
    };

    ui.label(RichText::new("●").size(14.0).color(color));
}

/// Vertical telemetry bar.
///
/// Used in:
/// - Pi node cards
/// - thermal visualization
/// - utilization indicators
pub fn vertical_bar(ui: &mut Ui, accent: Color32, progress: f32) {
    Frame::none()
        .fill(Color32::from_rgb(12, 18, 30))
        .stroke(Stroke::new(1.0, Color32::from_rgb(40, 50, 70)))
        .rounding(Rounding::same(12.0))
        .inner_margin(Margin::same(8.0))
        .show(ui, |ui| {
            ui.set_min_size(Vec2::new(36.0, 180.0));

            let filled_height = 140.0 * progress;

            ui.allocate_ui(Vec2::new(20.0, 150.0), |ui| {
                let rect = ui.max_rect();

                let fill_rect = egui::Rect::from_min_max(
                    egui::pos2(rect.min.x, rect.max.y - filled_height),
                    rect.max,
                );

                ui.painter().rect_filled(fill_rect, 8.0, accent);
            });
        });
}

/// Footer status widget.
///
/// Used for:
/// - uptime
/// - connectivity
/// - orchestration health
/// - cluster state
pub fn footer_indicator(ui: &mut Ui, icon: &str, accent: Color32) {
    Frame::none()
        .fill(Color32::from_rgb(6, 10, 18))
        .stroke(Stroke::new(1.0, Color32::from_rgb(30, 40, 60)))
        .rounding(Rounding::same(14.0))
        .inner_margin(Margin::same(12.0))
        .show(ui, |ui| {
            ui.set_min_size(Vec2::new(72.0, 64.0));

            ui.vertical_centered(|ui| {
                ui.label(RichText::new(icon).size(30.0).color(accent));
            });
        });
}

/// Top navigation icon button.
///
/// IMPORTANT:
/// This intentionally excludes text labels
/// to preserve appliance-style UI design.
pub fn nav_icon(ui: &mut Ui, icon: &str, selected: bool) -> egui::Response {
    let fill = if selected {
        Color32::from_rgb(10, 90, 180)
    } else {
        Color32::from_rgb(10, 14, 24)
    };

    let border = if selected {
        Color32::from_rgb(0, 180, 255)
    } else {
        Color32::from_rgb(40, 50, 70)
    };

    Frame::none()
        .fill(fill)
        .stroke(Stroke::new(1.2, border))
        .rounding(Rounding::same(16.0))
        .inner_margin(Margin::same(18.0))
        .show(ui, |ui| {
            ui.set_min_size(Vec2::new(96.0, 64.0));

            ui.vertical_centered(|ui| {
                ui.label(RichText::new(icon).size(34.0).color(Color32::WHITE));
            });
        })
        .response
}
