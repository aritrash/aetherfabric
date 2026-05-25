// runtime/ui/src/dashboard.rs

use egui::{
    Align, CentralPanel, Color32, Context, Frame, Layout, Margin, ProgressBar, RichText, Rounding,
    Stroke, TopBottomPanel, Ui, Vec2,
};

pub struct DashboardState {
    selected_tab: usize,
}

impl DashboardState {
    pub fn new() -> Self {
        Self { selected_tab: 0 }
    }

    pub fn render(&mut self, ctx: &Context) {
        self.render_top_bar(ctx);

        CentralPanel::default()
            .frame(
                Frame::none()
                    .fill(Color32::from_rgb(4, 8, 16))
                    .inner_margin(Margin::same(14.0)),
            )
            .show(ctx, |ui| {
                self.render_cluster_area(ui);

                ui.add_space(12.0);

                self.render_lower_panels(ui);

                ui.add_space(12.0);

                self.render_footer(ui);
            });
    }

    fn render_top_bar(&mut self, ctx: &Context) {
        TopBottomPanel::top("top_bar")
            .exact_height(96.0)
            .frame(
                Frame::none()
                    .fill(Color32::from_rgb(3, 6, 14))
                    .inner_margin(Margin::same(12.0)),
            )
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    let icons = ["⌂", "", "◫", "◎", "❄", "⚙"];

                    for (index, icon) in icons.iter().enumerate() {
                        let selected = self.selected_tab == index;

                        let fill = if selected {
                            Color32::from_rgb(10, 80, 180)
                        } else {
                            Color32::from_rgb(10, 14, 24)
                        };

                        let border = if selected {
                            Color32::from_rgb(0, 180, 255)
                        } else {
                            Color32::from_rgb(40, 50, 70)
                        };

                        let response = Frame::none()
                            .fill(fill)
                            .stroke(Stroke::new(1.5, border))
                            .rounding(Rounding::same(16.0))
                            .inner_margin(Margin::same(18.0))
                            .show(ui, |ui| {
                                ui.set_min_size(Vec2::new(110.0, 64.0));

                                ui.with_layout(Layout::top_down_justified(Align::Center), |ui| {
                                    ui.label(RichText::new(*icon).size(34.0).color(Color32::WHITE));
                                });
                            })
                            .response;

                        if response.clicked() {
                            self.selected_tab = index;
                        }

                        ui.add_space(10.0);
                    }
                });
            });
    }

    fn render_cluster_area(&self, ui: &mut Ui) {
        ui.horizontal(|ui| {
            self.render_pi_card(
                ui,
                "PI-1",
                0.58,
                0.72,
                0.52,
                Color32::from_rgb(40, 255, 120),
            );

            ui.add_space(12.0);

            self.render_pi_card(ui, "PI-2", 0.53, 0.50, 0.48, Color32::from_rgb(0, 170, 255));

            ui.add_space(12.0);

            self.render_pi_card(ui, "PI-3", 0.78, 0.89, 0.60, Color32::from_rgb(255, 170, 0));

            ui.add_space(12.0);

            self.render_esp_column(ui);
        });
    }

    fn render_pi_card(
        &self,
        ui: &mut Ui,
        title: &str,
        cpu: f32,
        ram: f32,
        temp: f32,
        accent: Color32,
    ) {
        Frame::none()
            .fill(Color32::from_rgb(8, 12, 22))
            .stroke(Stroke::new(1.2, accent))
            .rounding(Rounding::same(18.0))
            .inner_margin(Margin::same(14.0))
            .show(ui, |ui| {
                ui.set_min_size(Vec2::new(310.0, 320.0));

                ui.horizontal(|ui| {
                    ui.label(RichText::new("").size(32.0).color(accent));

                    ui.add_space(12.0);

                    ui.label(RichText::new(title).size(28.0).color(accent).strong());

                    ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                        ui.label(RichText::new("●").size(14.0).color(Color32::GREEN));
                    });
                });

                ui.add_space(18.0);

                self.metric_row(ui, "CPU", cpu, accent);

                ui.add_space(14.0);

                self.metric_row(ui, "RAM", ram, accent);

                ui.add_space(14.0);

                self.metric_row(ui, "TEMP", temp, accent);
            });
    }

    fn metric_row(&self, ui: &mut Ui, label: &str, value: f32, accent: Color32) {
        ui.horizontal(|ui| {
            ui.label(RichText::new(label).size(18.0).color(Color32::WHITE));

            ui.add_space(10.0);

            ui.add(ProgressBar::new(value).desired_width(180.0).fill(accent));

            ui.add_space(10.0);

            ui.label(
                RichText::new(format!("{:.0}%", value * 100.0))
                    .size(18.0)
                    .color(accent),
            );
        });
    }

    fn render_esp_column(&self, ui: &mut Ui) {
        ui.vertical(|ui| {
            self.render_esp_box(ui, 0.28);

            ui.add_space(10.0);

            self.render_esp_box(ui, 0.52);

            ui.add_space(10.0);

            self.render_esp_box(ui, 0.80);
        });
    }

    fn render_esp_box(&self, ui: &mut Ui, usage: f32) {
        Frame::none()
            .fill(Color32::from_rgb(16, 10, 24))
            .stroke(Stroke::new(1.0, Color32::from_rgb(170, 70, 255)))
            .rounding(Rounding::same(16.0))
            .inner_margin(Margin::same(12.0))
            .show(ui, |ui| {
                ui.set_min_size(Vec2::new(250.0, 88.0));

                ui.horizontal(|ui| {
                    ui.label(
                        RichText::new("◉")
                            .size(28.0)
                            .color(Color32::from_rgb(200, 90, 255)),
                    );

                    ui.add_space(12.0);

                    ui.add(
                        ProgressBar::new(usage)
                            .desired_width(160.0)
                            .fill(Color32::from_rgb(180, 70, 255)),
                    );
                });
            });
    }

    fn render_lower_panels(&self, ui: &mut Ui) {
        ui.horizontal(|ui| {
            self.render_cooling_panel(ui);

            ui.add_space(12.0);

            self.render_stat_panel(ui, Color32::from_rgb(255, 200, 0), "▤", 0.21);

            ui.add_space(12.0);

            self.render_stat_panel(ui, Color32::from_rgb(0, 255, 120), "◔", 0.53);
        });
    }

    fn render_cooling_panel(&self, ui: &mut Ui) {
        Frame::none()
            .fill(Color32::from_rgb(8, 12, 22))
            .stroke(Stroke::new(1.0, Color32::from_rgb(0, 180, 255)))
            .rounding(Rounding::same(18.0))
            .inner_margin(Margin::same(14.0))
            .show(ui, |ui| {
                ui.set_min_size(Vec2::new(520.0, 180.0));

                ui.horizontal(|ui| {
                    self.cooling_box(ui);

                    ui.add_space(12.0);

                    self.cooling_box(ui);
                });
            });
    }

    fn cooling_box(&self, ui: &mut Ui) {
        Frame::none()
            .fill(Color32::from_rgb(12, 18, 30))
            .stroke(Stroke::new(1.0, Color32::from_rgb(0, 120, 255)))
            .rounding(Rounding::same(14.0))
            .inner_margin(Margin::same(14.0))
            .show(ui, |ui| {
                ui.set_min_size(Vec2::new(210.0, 120.0));

                ui.vertical_centered(|ui| {
                    ui.label(
                        RichText::new("❄")
                            .size(34.0)
                            .color(Color32::from_rgb(0, 220, 255)),
                    );

                    ui.add_space(20.0);

                    ui.add(
                        ProgressBar::new(0.42)
                            .desired_width(140.0)
                            .fill(Color32::from_rgb(0, 150, 255)),
                    );
                });
            });
    }

    fn render_stat_panel(&self, ui: &mut Ui, accent: Color32, icon: &str, value: f32) {
        Frame::none()
            .fill(Color32::from_rgb(8, 12, 22))
            .stroke(Stroke::new(1.0, accent))
            .rounding(Rounding::same(18.0))
            .inner_margin(Margin::same(16.0))
            .show(ui, |ui| {
                ui.set_min_size(Vec2::new(250.0, 180.0));

                ui.vertical_centered(|ui| {
                    ui.label(RichText::new(icon).size(38.0).color(accent));

                    ui.add_space(24.0);

                    ui.add(ProgressBar::new(value).desired_width(170.0).fill(accent));

                    ui.add_space(12.0);

                    ui.label(
                        RichText::new(format!("{:.0}%", value * 100.0))
                            .size(26.0)
                            .color(accent)
                            .strong(),
                    );
                });
            });
    }

    fn render_footer(&self, ui: &mut Ui) {
        Frame::none()
            .fill(Color32::from_rgb(6, 10, 18))
            .stroke(Stroke::new(1.0, Color32::from_rgb(40, 50, 70)))
            .rounding(Rounding::same(18.0))
            .inner_margin(Margin::same(18.0))
            .show(ui, |ui| {
                ui.set_min_size(Vec2::new(ui.available_width(), 80.0));

                ui.horizontal_wrapped(|ui| {
                    let icons = [
                        ("✓", Color32::GREEN),
                        ("◎", Color32::from_rgb(0, 170, 255)),
                        ("❘", Color32::from_rgb(255, 120, 0)),
                        ("◷", Color32::LIGHT_GRAY),
                        ("◫", Color32::GRAY),
                    ];

                    for (icon, color) in icons {
                        ui.add_space(60.0);

                        ui.label(RichText::new(icon).size(36.0).color(color));
                    }
                });
            });
    }
}
