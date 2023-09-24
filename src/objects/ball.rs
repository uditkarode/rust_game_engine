use minifb::Key;

use crate::engine::{
    game_object::{CollisionShape, GameObject, GameObjectCommon},
    types::XYPair,
};

const KB_X_BOOST: f64 = 0.2;
const KB_Y_BOOST: f64 = 16.0;

pub struct Ball {
    radius: f64,
    diameter: f64,
    color: u32,

    common: GameObjectCommon,
}

impl Ball {
    pub fn new(coords: XYPair, radius: f64, color_hex: &str) -> Self {
        let color = u32::from_str_radix(&color_hex[1..], 16).unwrap_or(0xFFFFFF);
        let mut common = GameObjectCommon::new();
        common.set_coords(&coords);
        let diameter = radius * 2.0;

        Self {
            color,
            radius,
            diameter,

            common,
        }
    }
}

impl GameObject for Ball {
    fn weight_factor(&self) -> f64 {
        1.1
    }

    fn collision_shape(&self) -> CollisionShape {
        CollisionShape::Circle(self.radius)
    }

    fn common(&mut self) -> &mut GameObjectCommon {
        &mut self.common
    }

    fn draw(&self) -> Vec<Vec<u32>> {
        let mut raster = vec![vec![0; (self.radius * 2.0) as usize]; (self.radius * 2.0) as usize];
        let h = self.radius;
        let k = self.radius;

        for y in 0..(self.radius * 2.0) as usize {
            for x in 0..(self.radius * 2.0) as usize {
                let dx = (x as f64 - h).abs();
                let dy = (y as f64 - k).abs();
                if (dx * dx + dy * dy).sqrt() <= self.radius {
                    raster[y][x] = self.color;
                }
            }
        }

        raster
    }

    fn handle_input(&mut self, keys: &[Key]) {
        if keys.contains(&Key::A) {
            self.common.velocities.x -= KB_X_BOOST;
        }

        if keys.contains(&Key::D) {
            self.common.velocities.x += KB_X_BOOST;
        }

        // jump if we are on the ground AND have 0 or lesser y velocity
        if keys.contains(&Key::W) {
            if let Some(info) = &self.common.object_info {
                if self.common.velocities.y < 0.0
                    && self.common.coords.y + self.diameter == info.window_size.height as f64
                {
                    self.common.velocities.y -= KB_Y_BOOST;
                }
            }
        }
    }
}
