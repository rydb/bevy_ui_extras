use bevy_diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin, SystemInformationDiagnosticsPlugin};
use bevy_inspector_egui::egui::{Color32, FontFamily, FontId, RichText, Ui};
use colorgrad::Gradient;


pub mod plugins;
pub mod resources;

/// displays misc info for app status
/// !!! CPU/RAM usage stats do not work when dynamic linking is enabled !!!
pub fn display_app_status(
    //mut primary_window: Query<&mut EguiContext, With<PrimaryWindow>>,
    ui: &mut Ui,
    diagnostics: &DiagnosticsStore,
) {
    let gray = Color32::GRAY;
    let font = FontId::new(20.0, FontFamily::default());

    let fps_grad = colorgrad::GradientBuilder::new()
        .html_colors(&["deeppink", "gold", "seagreen"])
        .domain(&[0.0, 120.0])
        .build::<colorgrad::LinearGradient>()
        .unwrap();

    let rev_grad = colorgrad::GradientBuilder::new()
        .html_colors(&["seagreen", "gold", "deeppink"])
        .domain(&[0.0, 100.0])
        .build::<colorgrad::LinearGradient>()
        .unwrap();

    let fps = diagnostics
        .get(&FrameTimeDiagnosticsPlugin::FPS)
        .map(|diag| diag.value())
        .and_then(|n| n);

    let fps_color = fps
        .map(|n| {
            fps_grad
                .at(n as f32)
                .to_array()
                .map(|n| n * 255.0)
                .map(|n| n as u8)
        })
        .map_or_else(
            || gray,
            |n| Color32::from_rgba_unmultiplied(n[0], n[1], n[2], n[3]),
        );
    ui.horizontal(|ui| {
        ui.label(RichText::new("FPS:").font(font.clone()));
        ui.label(
            RichText::new(
                fps.map(|n| n.round().to_string())
                    .unwrap_or("???".to_owned()),
            )
            .color(fps_color)
            .font(font.clone()),
        )
    });

    let cpu_usage = diagnostics
        .get(&SystemInformationDiagnosticsPlugin::SYSTEM_CPU_USAGE)
        .map(|diag| diag.value())
        .and_then(|n| n);
    let cpu_color = cpu_usage
        .map(|n| {
            rev_grad
                .at(n as f32)
                .to_array()
                .map(|n| n * 255.0)
                .map(|n| n as u8)
        })
        .map_or_else(
            || gray,
            |n| Color32::from_rgba_unmultiplied(n[0], n[1], n[2], n[3]),
        );

    let horizontal = ui.horizontal(|ui| {
        ui.label(RichText::new("CPU usage:").font(font.clone()));
        ui.label(
            RichText::new(
                cpu_usage
                    .map(|n| n.round().to_string())
                    .unwrap_or("???".to_owned())
                    + "%",
            )
            .color(cpu_color)
            .font(font.clone()),
        )
    });
    let ram_usage = diagnostics
        .get(&SystemInformationDiagnosticsPlugin::SYSTEM_MEM_USAGE)
        .map(|diag| diag.value())
        .and_then(|n| n);

    let ram_color = ram_usage
        .map(|n| {
            rev_grad
                .at(n as f32)
                .to_array()
                .map(|n| n * 255.0)
                .map(|n| n as u8)
        })
        .map_or_else(
            || gray,
            |n| Color32::from_rgba_unmultiplied(n[0], n[1], n[2], n[3]),
        );
    ui.horizontal(|ui| {
        ui.label(RichText::new("RAM usage:").font(font.clone()));
        ui.label(
            RichText::new(
                ram_usage
                    .map(|n| n.round().to_string())
                    .unwrap_or("???".to_owned())
                    + "%",
            )
            .color(ram_color)
            .font(font.clone()),
        )
    });
}
