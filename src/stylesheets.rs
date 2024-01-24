use bevy::utils::default;
use egui::{epaint::Shadow, Color32, Frame, Margin, Rounding, Stroke};

///! default style sheets for specific "Looks"

pub const DEBUG_FRAME_STYLE: Frame = Frame {
    inner_margin: Margin {
        left: 0.0,
        right: 0.0,
        top: 0.0,
        bottom: 0.0,
    },
    outer_margin: Margin {
        left: 0.0,
        right: 0.0,
        top: 0.0,
        bottom: 0.0,
    },
    rounding: Rounding {
        nw: 0.0,
        ne: 0.0,
        sw: 0.0,
        se: 0.0,
    },
    shadow: Shadow::NONE,
    fill: egui::Color32::from_rgba_premultiplied(30,30,30, 128),
    stroke: Stroke::NONE
};