use crate::Axis;
use bevy::{
    color::{Color, Gray, Srgba},
    ecs::system::{Local, Res, SystemParam},
    gizmos::gizmos::Gizmos,
    math::Vec2,
    time::{Real, Time},
};
use std::{collections::BTreeMap, time::Duration};

/// A helper struct for visualization of the axis values. It takes in an axis X and an optional axis Y which can be used
/// to visualize axis behaviour using gizmos.
#[derive(SystemParam)]
pub struct AxisVisualizer<'w, 's, X: Send + Sync + 'static, Y: Send + Sync + 'static = ()> {
    gizmos: Gizmos<'w, 's>,
    axis_x: Option<Res<'w, Axis<X>>>,
    axis_y: Option<Res<'w, Axis<Y>>>,
    time: Res<'w, Time<Real>>,
    stored: Local<'s, BTreeMap<Duration, Vec2>>,
}

impl<X: Send + Sync + 'static, Y: Send + Sync + 'static> AxisVisualizer<'_, '_, X, Y> {
    fn store_current(&mut self) {
        let now = self.time.elapsed();
        let x = self.axis_x.as_ref().map_or(0.0, |axis| axis.value());
        let y = self.axis_y.as_ref().map_or(0.0, |axis| axis.value());
        self.stored.insert(now, Vec2::new(x, y));
    }

    /// Graphs the X axis values over time.
    pub fn graph_x(
        &mut self,
        timespan: Duration,
        position: Vec2,
        scale: f32,
        size: Vec2,
        color: impl Into<Color>,
    ) -> &mut Self {
        self.store_current();

        let since_start = self.time.elapsed();

        let values = self
            .stored
            .iter()
            .filter_map(|(timestamp, point)| {
                let elapsed = since_start - *timestamp;
                if elapsed < timespan {
                    Some((elapsed.as_secs_f32(), point.x))
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();

        let points = values
            .iter()
            .map(|(time, value)| {
                Vec2::new(
                    size.x - (time / timespan.as_secs_f32()) * size.x - size.x * 0.5 + position.x,
                    size.y / 2.0 + value * scale - size.y * 0.5 + position.y,
                )
            })
            .collect::<Vec<_>>();

        self.gizmos.linestrip_2d(points, color);

        self
    }

    /// Graphs the Y axis values over time.
    pub fn graph_y(
        &mut self,
        timespan: Duration,
        position: Vec2,
        scale: f32,
        size: Vec2,
        color: impl Into<Color>,
    ) -> &mut Self {
        self.store_current();

        let since_start = self.time.elapsed();

        let values = self
            .stored
            .iter()
            .filter_map(|(timestamp, point)| {
                let elapsed = since_start - *timestamp;
                if elapsed < timespan {
                    Some((elapsed.as_secs_f32(), point.y))
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();

        let points = values
            .iter()
            .map(|(time, value)| {
                Vec2::new(
                    size.x - (time / timespan.as_secs_f32()) * size.x - size.x * 0.5 + position.x,
                    size.y / 2.0 + value * scale - size.y * 0.5 + position.y,
                )
            })
            .collect::<Vec<_>>();

        self.gizmos.linestrip_2d(points, color);

        self
    }

    /// Draws a circle representing the current X and Y axis values.
    pub fn axis_circle(
        &mut self,
        position: Vec2,
        scale: f32,
        radius: f32,
        color: impl Into<Color>,
    ) -> &mut Self {
        self.store_current();

        let Some((_, point)) = self.stored.iter().next_back() else {
            return self;
        };
        let point_position = point * scale + position;

        self.gizmos.circle_2d(position, radius, Srgba::gray(0.7));
        self.gizmos.circle_2d(point_position, 8.0, color);

        self
    }
}
