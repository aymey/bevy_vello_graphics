use bevy::{math::DVec2, prelude::*};
use bevy_vello::vello::{self, kurbo};

use crate::{Fill, Stroke};

#[allow(clippy::type_complexity)]
pub(super) fn draw_vectors<V: Vector + Component>(
    mut commands: Commands,
    q_vectors: Query<
        (Entity, &V, Option<&Fill>, Option<&Stroke>),
        Or<(Changed<V>, Changed<Fill>, Changed<Stroke>)>,
    >,
) {
    for (entity, vector, fill, stroke) in q_vectors.iter() {
        let mut scene = vello::Scene::new();

        if let Some(fill) = fill {
            scene.fill(
                fill.style,
                kurbo::Affine::IDENTITY,
                &fill.brush.value,
                Some(fill.brush.transform),
                &vector.shape(),
            );
        }

        if let Some(stroke) = stroke {
            scene.stroke(
                &stroke.style,
                kurbo::Affine::IDENTITY,
                &stroke.brush.value,
                Some(stroke.brush.transform),
                &vector.shape(),
            );
        }

        commands.entity(entity).insert(VectorScene(scene));
    }
}

#[derive(Component, Default, Clone)]
pub struct VectorScene(pub vello::Scene);

pub trait Vector {
    /// Returns vector graphics that implements [`kurbo::Shape`].
    fn shape(&self) -> impl kurbo::Shape;
    /// Translation of the border at a specific `time` value.
    fn border_translation(&self, time: f64) -> DVec2;
    /// The rotation at the tangent of the border at a specific `time` value.
    fn border_rotation(&self, time: f64) -> f64;
}
