//! Drop-in egui update panel. Rendering/backend and persistent settings stay with
//! the application. `style` is optional and never applied by `show` or `start`.
pub mod style;

use egui::RichText;
use rust_auto_updator::{Config, Controller, State};

/// Connect worker events to egui's existing event loop, without polling frames.
pub fn start(config: Config, ctx: egui::Context) -> Result<Controller, String> {
    Controller::new(config, move || {
        ctx.request_repaint_of(egui::ViewportId::ROOT)
    })
}

#[derive(Default)]
pub struct Response {
    pub preference_changed: bool,
}

/// Draw using the host's current egui theme. The host persists `check_on_startup`
/// when the returned response reports a change. Call `Controller::poll` in the
/// host logic loop, including while this panel is closed.
pub fn show(ui: &mut egui::Ui, updates: &mut Controller, check_on_startup: &mut bool) -> Response {
    let mut response = Response::default();
    egui::Frame::new()
        .fill(ui.visuals().window_fill)
        .stroke(ui.visuals().widgets.noninteractive.bg_stroke)
        .corner_radius(9)
        .inner_margin(16)
        .show(ui, |ui| {
            ui.set_min_width(ui.available_width());
            ui.horizontal(|ui| {
                ui.label(RichText::new("Updates").size(17.0).strong());
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    if ui
                        .add_enabled(
                            updates.enabled() && !updates.state().busy(),
                            egui::Button::new("Check for updates"),
                        )
                        .clicked()
                    {
                        updates.check();
                    }
                });
            });
            ui.label(
                RichText::new(format!("Installed version {}", updates.installed_version()))
                    .size(11.0)
                    .color(ui.visuals().weak_text_color()),
            );
            ui.add_space(8.0);
            let muted = ui.visuals().weak_text_color();
            match updates.state() {
                State::Idle => {
                    ui.label(
                        RichText::new(updates.disabled_reason().unwrap_or(
                            "Updates are downloaded from the official GitHub releases.",
                        ))
                        .size(12.0)
                        .color(muted),
                    );
                }
                State::Checking => {
                    ui.horizontal(|ui| {
                        ui.spinner();
                        ui.label("Checking GitHub…");
                    });
                }
                State::Current => {
                    ui.label(RichText::new("You're up to date.").size(13.0).color(muted));
                }
                State::Available(release) => {
                    let version = release.version.to_string();
                    let page = release.page.clone();
                    let notes = release.notes.clone();
                    ui.label(format!("Version {version} is available"));
                    ui.horizontal(|ui| {
                        if ui.button("Download and install").clicked() {
                            updates.download_available();
                        }
                        ui.hyperlink_to("Release notes on GitHub", page);
                    });
                    ui.label(
                        RichText::new("The app will restart. Your settings will be kept.")
                            .size(11.0)
                            .color(muted),
                    );
                    if !notes.is_empty() {
                        ui.collapsing("What's new", |ui| {
                            ui.label(notes);
                        });
                    }
                }
                State::Downloading { received, total } => {
                    let progress = if *total > 0 {
                        *received as f32 / *total as f32
                    } else {
                        0.0
                    };
                    ui.add(
                        egui::ProgressBar::new(progress)
                            .show_percentage()
                            .animate(true),
                    );
                    ui.label(
                        RichText::new("Downloading and verifying SHA-256…")
                            .size(12.0)
                            .color(muted),
                    );
                }
                State::Installing => {
                    ui.label("Restarting to finish the update…");
                }
                State::Error(error) => {
                    ui.label(
                        RichText::new(error)
                            .size(12.0)
                            .color(ui.visuals().warn_fg_color),
                    );
                }
            }
            ui.add_space(8.0);
            response.preference_changed = ui
                .add_enabled(
                    updates.enabled(),
                    egui::Checkbox::new(check_on_startup, "Check for updates on startup"),
                )
                .changed();
        });
    response
}
