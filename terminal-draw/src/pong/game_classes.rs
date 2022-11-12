pub mod paddle {
    use crate::{coordinate::coordinate::Coordinate, surface::surface::{Surface, blit}};

    pub struct Paddle {
        pub height: u32,
        pub position: Coordinate,
        pub surface: Surface,
    }
    impl Paddle {
        pub fn new(height: u32, x_pos: i32, fill: char) -> Self {
            Self {
                height,
                position: Coordinate::new(x_pos, 0),
                surface: Surface::new(1, height, fill)
            }
        }
        pub fn draw(&self, dest_surf: &mut Surface) {
            blit(self.surface.clone(), dest_surf, self.position);
        }
        pub fn move_up(&mut self) {
            self.position.y -= 1;
        }
        pub fn move_down(&mut self) {
            self.position.y += 1;
        }
    }
}

pub mod velocity {
    pub struct Velocity {
        pub x: i32,
        pub y: i32,
    }
    impl Velocity {
        pub fn new(x: i32, y: i32) -> Self {
            Self {
                x,
                y
            }
        }
    }
}

pub mod ball {
    use crate::coordinate::coordinate::Coordinate;
    use crate::pong::game_classes::velocity::Velocity;
    use crate::surface::surface::{Surface, blit, DrawChar};
    pub struct Ball {
        pub position: Coordinate,
        pub velocity: Velocity,
        pub surface: Surface,
    }
    impl Ball {
        pub fn apply_velocity (&mut self) {
            // 'move' was taken already
            self.position.x = self.position.x + self.velocity.x; 
            self.position.y = self.position.y + self.velocity.y;
        }
        pub fn draw(&self, dest_surf: &mut Surface) {
            blit(self.surface.clone(), dest_surf, self.position);
        }
        pub fn new(starting_position: Coordinate, starting_velocity: Velocity) -> Self {
            let mut new_surface = Surface::new(2, 1, '(');
            new_surface.draw_char(Coordinate::new(1, 0), ')');
            Self {
                position: starting_position,
                velocity: starting_velocity,
                surface: new_surface,
            }
        }
        pub fn bounce(&mut self, x: bool, y: bool) {
            if x {
                self.velocity.x *= -1;
            }
            if y {
                self.velocity.y *= -1;
            }
        }
    }
}