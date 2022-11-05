pub mod coordinate {
    use std::fmt;

    pub struct Coordinate {
        pub x: u32,
        pub y: u32,
    }
    impl Coordinate {
        pub fn new(x: u32, y: u32) -> Self {
            Self {
                x,
                y,
            }
            
        }
        pub fn reposition(&mut self, new_x: u32, new_y: u32) {
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