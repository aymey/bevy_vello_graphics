use bevy::{math::DVec2, prelude::*};
use bevy_vello::prelude::*;

use crate::VectorBorder;

use super::VelloVector;

#[derive(Component, Default, Debug, Clone, Copy)]
pub struct VelloRect {
    pub size: DVec2,
    pub anchor: DVec2,
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

impl VelloVector for VelloRect {
    fn shape(&self) -> impl kurbo::Shape {
        kurbo::RoundedRect::new(
            -self.size.x * self.anchor.x,
            -self.size.y * self.anchor.y,
            self.size.x * (1.0 - self.anchor.x),
            self.size.y * (1.0 - self.anchor.y),
            self.radius,
        )
    }
}

impl VectorBorder for VelloRect {
    fn border_translation(&self, _time: f32) -> DVec2 {
        DVec2::new(self.anchor.x, (1.0 - self.anchor.y) * self.size.y)
    }

    fn border_tangent(&self, _time: f32) -> f64 {
        0.0
    }
}
