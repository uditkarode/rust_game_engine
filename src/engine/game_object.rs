use minifb::Key;

use super::types::{ObjectInfo, XYPair};

pub enum CollisionShape {
    Circle(f64),
}

pub struct GameObjectCommon {
    pub coords: XYPair,
    pub velocities: XYPair,
    pub object_info: Option<ObjectInfo>,
}

impl GameObjectCommon {
    pub fn new() -> Self {
        Self {
            coords: XYPair { x: 0.0, y: 0.0 },
            velocities: XYPair { x: 0.0, y: 0.0 },
            object_info: None,
        }
    }

    pub fn get_coords(&self) -> &XYPair {
        &self.coords
    }
    pub fn set_coords(&mut self, coords: &XYPair) {
        self.coords = coords.clone();
    }

    pub fn get_velocities(&self) -> &XYPair {
        &self.velocities
    }
    pub fn set_velocities(&mut self, velocities: &XYPair) {
        self.velocities = velocities.clone();
    }

    pub fn set_object_info(&mut self, object_info: &ObjectInfo) {
        self.object_info = Some(object_info.clone());
    }
}

pub trait GameObject {
    fn common(&mut self) -> &mut GameObjectCommon;
    fn weight_factor(&self) -> f64;
    fn draw(&self) -> Vec<Vec<u32>>;
    fn collision_shape(&self) -> CollisionShape;
    fn handle_input(&mut self, _keys: &[Key]) {}
}
