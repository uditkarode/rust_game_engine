use minifb::Key;

use super::{
    constants::DEFAULT_COLLISION_DAMPING_FACTOR,
    types::{ObjectInfo, XYPair},
};

// collision shape
pub enum CollisionShape {
    Circle(f64),
}

impl CollisionShape {
    pub fn effective_size(&self) -> XYPair {
        match self {
            Self::Circle(radius) => XYPair {
                x: radius * 2.0,
                y: radius * 2.0,
            },
        }
    }
}

// game object common
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

// game object
pub trait GameObject {
    fn common(&mut self) -> &mut GameObjectCommon;

    fn weight_factor(&self) -> f64;

    fn bounciness(&self) -> f64 {
        DEFAULT_COLLISION_DAMPING_FACTOR
    }

    fn collision_shape(&self) -> CollisionShape;

    fn draw(&self) -> Vec<Vec<u32>>;

    fn handle_input(&mut self, _keys: &[Key]) {}
}
