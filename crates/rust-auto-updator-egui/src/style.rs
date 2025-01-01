//! Optional application-wide theme, tokens and shared containers.
//! The update panel never changes the host context theme.
use egui::{Color32, FontId, Stroke, Vec2};
use std::path::PathBuf;

pub const BG: Color32 = Color32::from_rgb(23, 24, 27);
pub const CARD: Color32 = Color32::from_rgb(32, 33, 38);
pub const BORDER: Color32 = Color32::from_rgb(51, 53, 60);
pub const ACCENT: Color32 = Color32::from_rgb(137, 167, 237);
pub const TRACK: Color32 = Color32::from_rgb(66, 68, 77);
pub const TEXT: Color32 = Color32::from_rgb(239, 239, 242);
pub const MUTED: Color32 = Color32::from_rgb(162, 164, 175);
pub const WARM: Color32 = Color32::from_rgb(224, 181, 132);

pub fn apply_dark_theme(ctx: &egui::Context) {
    // Use the installed Windows UI font without bundling a proprietary font file.
    if let Some(windows) = std::env::var_os("WINDIR")
        && let Ok(bytes) = std::fs::read(PathBuf::from(windows).join("Fonts").join("segoeui.ttf"))
    {
        let mut fonts = egui::FontDefinitions::default();
        fonts.font_data.insert(
            "windows-ui".into(),
            egui::FontData::from_owned(bytes).into(),
        );
        fonts
            .families
            .entry(egui::FontFamily::Proportional)
            .or_default()
            .insert(0, "windows-ui".into());
        ctx.set_fonts(fonts);
    }
    let mut style = (*ctx.style_of(egui::Theme::Dark)).clone();
    style.visuals = egui::Visuals::dark();
    style.visuals.override_text_color = Some(TEXT);
    style.visuals.panel_fill = BG;
    style.visuals.window_fill = CARD;
    style.visuals.extreme_bg_color = BG;
    style.visuals.faint_bg_color = CARD;
    style.visuals.hyperlink_color = ACCENT;
    style.visuals.selection.bg_fill = Color32::from_rgb(51, 66, 96);
    style.visuals.selection.stroke = Stroke::new(1.0, ACCENT);
    style.visuals.widgets.noninteractive.fg_stroke = Stroke::new(1.0, TEXT);
    style.visuals.widgets.noninteractive.bg_stroke = Stroke::new(1.0, BORDER);
    style.visuals.widgets.inactive.bg_fill = Color32::from_rgb(43, 44, 51);
    style.visuals.widgets.inactive.weak_bg_fill = Color32::from_rgb(43, 44, 51);
    style.visuals.widgets.inactive.bg_stroke = Stroke::new(1.0, BORDER);
    style.visuals.widgets.inactive.fg_stroke = Stroke::new(1.0, MUTED);
    style.visuals.widgets.hovered.bg_fill = Color32::from_rgb(53, 55, 64);
    style.visuals.widgets.hovered.weak_bg_fill = Color32::from_rgb(53, 55, 64);
    style.visuals.widgets.hovered.bg_stroke = Stroke::new(1.0, Color32::from_rgb(81, 86, 103));
    style.visuals.widgets.hovered.fg_stroke = Stroke::new(1.0, TEXT);
    style.visuals.widgets.active.bg_fill = Color32::from_rgb(58, 66, 88);
    style.visuals.widgets.active.fg_stroke = Stroke::new(1.2, TEXT);
    for widgets in [
        &mut style.visuals.widgets.inactive,
        &mut style.visuals.widgets.hovered,
        &mut style.visuals.widgets.active,
    ] {
        widgets.corner_radius = 5.into();
    }
    style.spacing.item_spacing = Vec2::new(8.0, 6.0);
    style.spacing.button_padding = Vec2::new(10.0, 5.0);
    style.spacing.interact_size.y = 25.0;
    style.spacing.icon_width = 15.0;
    style.spacing.scroll = egui::style::ScrollStyle::solid();
    style.spacing.scroll.bar_width = 6.0;
    style
        .text_styles
        .insert(egui::TextStyle::Body, FontId::proportional(13.0));
    style
        .text_styles
        .insert(egui::TextStyle::Button, FontId::proportional(12.0));
    ctx.set_theme(egui::Theme::Dark);
    ctx.set_style_of(egui::Theme::Dark, style);
}

pub fn card(ui: &mut egui::Ui, contents: impl FnOnce(&mut egui::Ui)) {
    egui::Frame::new()
        .fill(CARD)
        .stroke(Stroke::new(1.0, BORDER))
        .corner_radius(9)
        .inner_margin(16)
        .show(ui, |ui| {
            ui.set_min_width(ui.available_width());
            contents(ui);
        });
}
