use bevy::{math::DVec2, prelude::*};
use bevy_vello::prelude::*;

use super::VelloVector;

#[derive(Component, Default, Debug, Clone, Copy)]
pub struct VelloArrow {
    pub head: ArrowHead,
    pub size: f32,
    pub offset: f32,
    pub rotation: f32,
}

#[derive(Default, Debug, Clone, Copy)]
pub enum ArrowHead {
    #[default]
    Triangle,
    Square,
    Circle,
}

impl VelloArrow {
    pub fn new(head: ArrowHead, size: f32, offset: f32, rotation: f32) -> Self {
        Self {
            head,
            size,
            offset,
            rotation,
        }
    }

    pub fn new_simple(head: ArrowHead, size: f32) -> Self {
        Self {
            head,
            size,
            ..default()
        }
    }

    pub fn with_head(mut self, head: ArrowHead) -> Self {
        self.head = head;
        self
    }

    pub fn with_size(mut self, size: f32) -> Self {
        self.size = size;
        self
    }

    pub fn with_offset(mut self, offset: f32) -> Self {
        self.offset = offset;
        self
    }

    pub fn with_rotation(mut self, angle: f32) -> Self {
        self.rotation = angle;
        self
    }
}

impl VelloVector for VelloArrow {
    fn shape(&self) -> impl kurbo::Shape {
        // kurbo::Triangle::new()
        todo!();
    }
}
