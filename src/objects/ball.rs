use minifb::Key;

use crate::engine::{
    game_object::{CollisionShape, GameObject},
    types::Coords,
};

pub struct Ball {
    coords: Coords,
    radius: f64,
    color: u32,
}

impl Ball {
    pub fn new(coords: Coords, radius: f64, color_hex: &str) -> Self {
        let color = u32::from_str_radix(&color_hex[1..], 16).unwrap_or(0xFFFFFF);
        Self {
            coords,
            radius,
            color,
        }
    }
}

impl GameObject for Ball {
    fn weight_factor(&self) -> f64 {
        1.2
    }

    fn collision_shape(&self) -> CollisionShape {
        return CollisionShape::Circle(self.radius);
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

    fn get_coords(&self) -> &Coords {
        &self.coords
    }

    fn set_coords(&mut self, coords: Coords) {
        self.coords = coords;
    }

    fn handle_input(&mut self, keys: &[Key]) {
        if keys.contains(&Key::A) {}
        if keys.contains(&Key::D) {}
        if keys.contains(&Key::W) {}
    }
}
