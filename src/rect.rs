//! A Bevy friendly wrapper around [`kurbo::RoundedRect`].

use bevy_ecs::prelude::*;
use bevy_math::DVec2;
use bevy_vello::vello::kurbo;

use super::Vector;

/// Vello rect component.
#[derive(Component, Default, Debug, Clone, Copy)]
pub struct VelloRect {
    /// Width and height.
    pub size: DVec2,
    /// Origin of the rect.
    pub anchor: DVec2,
    /// Border radius.
    pub radius: f64,
}

impl VelloRect {
    pub fn new(width: f64, height: f64) -> Self {
        Self {
            size: DVec2::new(width, height),
            anchor: DVec2::splat(0.5),
            radius: 0.0,
        }
    }

    pub fn with_size(mut self, width: f64, height: f64) -> Self {
        self.size = DVec2::new(width, height);
        self
    }

    pub fn with_anchor(mut self, x: f64, y: f64) -> Self {
        self.anchor = DVec2::new(x, y);
        self
    }

    pub fn with_radius(mut self, radius: f64) -> Self {
        self.radius = radius;
        self
    }
}

impl Vector for VelloRect {
    fn shape(&self) -> impl kurbo::Shape {
        kurbo::RoundedRect::new(
            -self.size.x * self.anchor.x,
            -self.size.y * self.anchor.y,
            self.size.x * (1.0 - self.anchor.x),
            self.size.y * (1.0 - self.anchor.y),
            self.radius,
        )
    }

    fn border_translation(&self, _time: f64) -> DVec2 {
        DVec2::ZERO
    }

    fn border_rotation(&self, _time: f64) -> f64 {
        0.0
    }
}
