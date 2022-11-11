pub mod coordinate {
    use std::fmt;

    #[derive(Copy, Clone)]
    pub struct Coordinate {
        pub x: i32,
        pub y: i32,
    }
    impl Coordinate {
        pub fn new(x: i32, y: i32) -> Self {
            Self {
                x,
                y,
            }
            
        }
        pub fn reposition(&mut self, new_x: i32, new_y: i32) {
            self.x = new_x;
            self.y = new_y;
        }
        // I'll probably add an apply_velocity function later
    }
    // debugging purposes only
    impl fmt::Display for Coordinate {
		fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
			write!(f, "({}, {})", self.x, self.y)
		}
	}
}