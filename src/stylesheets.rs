use bevy_inspector_egui::egui::{self, CornerRadius};
use egui::{Color32, Frame, Margin, Stroke, epaint::Shadow};

pub const DEBUG_FRAME_STYLE: Frame = Frame {
    inner_margin: Margin {
        left: 0,
        right: 0,
        top: 0,
        bottom: 0,
    },
    outer_margin: Margin {
        left: 0,
        right: 0,
        top: 0,
        bottom: 0,
    },
    corner_radius: CornerRadius {
        nw: 0,
        ne: 0,
        sw: 0,
        se: 0,
    },
    shadow: Shadow::NONE,
    fill: egui::Color32::from_rgba_premultiplied(15, 15, 15, 128),
    stroke: Stroke {
        width: 1.0,
        color: Color32::BLACK,
    },
};
