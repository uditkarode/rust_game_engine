use minifb::{Key, ScaleMode, Window, WindowOptions};

use super::{
    constants::*,
    game_object::{CollisionShape, GameObject},
    types::{ObjectInfo, WindowSize, XYPair},
};

pub struct Engine {
    window: Option<Window>,
    buffer: Vec<u32>,
    window_size: WindowSize,
    objects: Vec<Box<dyn GameObject>>,
}

// public functions
impl Engine {
    pub fn new(window_size: &WindowSize) -> Result<Self, anyhow::Error> {
        Ok(Self {
            buffer: vec![0; window_size.width * window_size.height],
            window: None,
            window_size: window_size.clone(),
            objects: Vec::new(),
        })
    }

    pub fn add_game_object(&mut self, game_object: impl GameObject + 'static) {
        self.objects.push(Box::new(game_object))
    }
}

// internal functions
impl Engine {
    fn calc_velocities(object: &mut Box<dyn GameObject>) {
        let mut velocities = object.common().get_velocities().clone();

        // apply gravity
        let gravity = GRAVITY * object.weight_factor() * DT;
        velocities.y += gravity;

        // apply air drag
        velocities.x *= 1.0 - (AIR_RESISTANCE_FACTOR * DT);
        velocities.y *= 1.0 - (AIR_RESISTANCE_FACTOR * DT);

        object.common().set_velocities(&velocities);
    }

    fn apply_velocities(object: &mut Box<dyn GameObject>) {
        let common = object.common();
        let coords = common.get_coords().clone();
        let velocities = common.get_velocities().clone();

        object.common().set_coords(&XYPair {
            x: coords.x + velocities.x,
            y: coords.y + velocities.y,
        });
    }

    fn collision_checks(window_size: &WindowSize, object: &mut Box<dyn GameObject>) {
        match object.collision_shape() {
            CollisionShape::Circle(radius) => {
                let mut coords = object.common().get_coords().clone();
                let mut velocities = object.common().get_velocities().clone();
                let diameter = 2.0 * radius;

                let on_ground = if coords.y + diameter >= window_size.height as f64 {
                    true
                } else {
                    false
                };

                let on_x_collision =
                    |velocities: &mut XYPair| velocities.x = -velocities.x * object.bounciness();

                let on_y_collision = |velocities: &mut XYPair| {
                    velocities.y = -velocities.y * object.bounciness();

                    // if we're just rolling on the ground, apply ground drag
                    if on_ground && velocities.y.abs() <= 1.0 {
                        velocities.x -= velocities.x * GROUND_DRAG_FACTOR
                    }
                };

                // x axis window collision
                if coords.x <= 0.0 {
                    coords.x = 0.0;
                    on_x_collision(&mut velocities);
                }
                if coords.x + diameter > window_size.width as f64 {
                    coords.x = window_size.width as f64 - diameter;
                    on_x_collision(&mut velocities);
                }

                // y axis window collision
                if coords.y - diameter < 0.0 {
                    coords.y = diameter;
                    on_y_collision(&mut velocities);
                }
                if coords.y + diameter > window_size.height as f64 {
                    coords.y = window_size.height as f64 - diameter;
                    on_y_collision(&mut velocities);
                }

                object.common().set_coords(&coords);
                object.common().set_velocities(&velocities);
            }
        }
    }

    fn update_object_info(window_size: &WindowSize, object: &mut Box<dyn GameObject>) {
        let object_info = ObjectInfo {
            window_size: window_size.clone(),
        };

        object.common().set_object_info(&object_info);
    }

    fn draw(buffer: &mut Vec<u32>, window_size: &WindowSize, object: &mut Box<dyn GameObject>) {
        let raster_vecs = object.draw();

        let common = object.common();
        let coords = &common.get_coords();

        Engine::draw_at(
            buffer,
            window_size.width,
            window_size.height,
            raster_vecs,
            coords,
        );
    }
}

// internal utils
impl Engine {
    fn draw_at(
        buffer: &mut Vec<u32>,
        buffer_width: usize,
        buffer_height: usize,
        raster_vecs: Vec<Vec<u32>>,
        coords: &XYPair,
    ) {
        let object_width = raster_vecs.iter().map(|row| row.len()).max().unwrap_or(0);

        for (dy, row) in raster_vecs.iter().enumerate() {
            for dx in 0..object_width {
                let x = (coords.x + dx as f64) as usize;
                let y = (coords.y + dy as f64) as usize;

                // make sure this is not out of the buffer
                if x < buffer_width && y < buffer_height {
                    let index = y * buffer_width + x;

                    let maybe_pixel = row.get(dx).cloned();
                    if let Some(pixel) = maybe_pixel {
                        buffer[index] = pixel;
                    }
                }
            }
        }
    }
}

// main run function -- sets up the window and the game loop
impl Engine {
    pub fn run(&mut self, window_title: &str) -> Result<(), anyhow::Error> {
        self.window = Some(Window::new(
            window_title,
            self.window_size.width,
            self.window_size.height,
            WindowOptions {
                scale_mode: ScaleMode::AspectRatioStretch,
                ..WindowOptions::default()
            },
        )?);

        let duration_per_frame = std::time::Duration::from_secs(1) / FPS.try_into()?;
        self.window
            .as_mut()
            .unwrap()
            .limit_update_rate(Some(duration_per_frame));

        while self.window.as_ref().unwrap().is_open()
            && !self.window.as_ref().unwrap().is_key_down(Key::Escape)
        {
            let start_time = std::time::Instant::now();
            let keys = self.window.as_ref().unwrap().get_keys();

            // clear the display buffer
            self.buffer.iter_mut().for_each(|p| *p = 0);

            for object in self.objects.iter_mut() {
                // re-calculate the velocities of the object
                Engine::calc_velocities(object);

                // apply the velocities to the coordinates
                Engine::apply_velocities(object);

                // perform collision checks with the window
                Engine::collision_checks(&self.window_size, object);

                // update the game object's info
                Engine::update_object_info(&self.window_size, object);

                // allow the object to react to pressed keys
                object.handle_input(&keys);

                // draw the object on the buffer at it's coords
                Engine::draw(&mut self.buffer, &self.window_size, object);
            }

            // reflect the display buffer changes
            self.window.as_mut().unwrap().update_with_buffer(
                &self.buffer,
                self.window_size.width,
                self.window_size.height,
            )?;

            // we've done everything we needed to this frame,
            // so we can sleep until the next frame is needed.
            std::thread::sleep(
                std::time::Duration::from_secs_f64(DT).saturating_sub(start_time.elapsed()),
            );
        }

        Ok(())
    }
}
