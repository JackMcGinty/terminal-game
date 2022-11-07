pub mod surface {
    use crate::coordinate::coordinate::Coordinate;
    pub struct Surface {
        pub width: u32,
        pub height: u32,
        pub default_char: char,
        pub contents: Vec<Vec<char>>
    }


    // Won't have time to finish this right now.
    //  the plan is to have all the functions owned 
    //  by Surface as traits so that a screen object
    //  can wrap them up and call them conveniently.
    pub trait Fill {
        fn fill(&mut self, new_char: char);
        }
    pub trait Clear {
        fn clear(&mut self);
    }
    pub trait DrawChar {
        fn draw_char(&mut self, dest_coord: Coordinate, dest_char: char);
    }
    pub trait VerifyPoint {
        fn verify_point(&self, test_point: &Coordinate) -> bool;
    }
    pub trait Display {
        fn display(&self);
    }
    // write some text to the surface (useful for HUDs, menus, and so forth)
    pub trait Write {
        fn write(&mut self, source: String, dest: Coordinate);
    }
    // draw a line from point a to point b
    pub trait DrawLine {
        fn draw_line(&mut self, a: Coordinate, b: Coordinate, line_char: char);
    }

// There's also new() as a shared behavior but it is defined differently
//  so I can't really have it as a trait.

    impl Surface {
        // Constructor
        pub fn new(width: u32, height: u32, default_char: char) -> Self {
            let mut contents: Vec<Vec<char>> = Vec::new();
            // row needs to be usize so we can index a vector with it
            for row in 0..(usize::try_from(height).unwrap())  {
                contents.push(Vec::new());
                // this \/ is why we need row as a usize
                for _col in 0..width { // underscore because it is a counter and is never assessed
                    contents[row].push(default_char);
                }
            }
            Self {
                width,
                height,
                default_char,
                contents
            }
        }
    }

    // fill the surface with the argued char
    impl Fill for Surface {
        fn fill(&mut self, new_char: char) {
            for row in 0..self.contents.len() {
                for col in 0..self.contents[row].len() {
                    self.contents[row][col] = new_char;
                }
            }
        }
    }

    // reset everything to the default char
    impl Clear for Surface {
        fn clear(&mut self) {
            for row in 0..self.contents.len() {
                for col in 0..self.contents[row].len() {
                    self.contents[row][col] = self.default_char;
                }
            }
        }
    }

    // change a char at the argued (x, y) coordinate
    impl DrawChar for Surface {
        fn draw_char(&mut self, dest_coord: Coordinate, dest_char: char) {
            // the secret to Rust programming is that when you get
            //  ownership errors, you just throw ampersands in front of 
            //  stuff until the errors go away. 
            // It's like C++ and dereference operators
            if !self.verify_point(&dest_coord) {
                return // bail from function if we get a faulty coordinate
                // there's probably some cool Rust error-checking stuff
                //  to be done here
            }
            let useful_x = usize::try_from(dest_coord.x).unwrap();
            let useful_y = usize::try_from(dest_coord.y).unwrap();
            self.contents[useful_y][useful_x] = dest_char;
            // We have to index y first because of the nature of the code
            //  Fortuantely, because of data shielding, I don't ever have to
            //  think about this in the code (except right here)
        }
    }

    // verify a point (primarily for internal use)
    impl VerifyPoint for Surface {
        fn verify_point(&self, test_point: &Coordinate) -> bool {
            // since test_point.x and .y are unsigned, they will 
            //  never be negative.
            //  i.e. the coordinate is always in-bounds from 
            //  the top and left. I'd really like to say that I thought
            //  of this but it was honestly an accident.
            //  Also the fact that Rust detects this is absolutely
            //  butt-kicking boss.
            if ((self.width - 1) < test_point.x) || 
                ((self.height - 1) < test_point.y) {
                return false
            }
            // if we reach it this far, the coordinate must be valid
            true

        }
    }

    // show on screen (mostly debug for the base Surface,
    //  but much more important for the Screen class)
    impl Display for Surface {
        fn display(&self) {
            for row in 0..self.contents.len() {
                for col in 0..self.contents[row].len() {
                    print!("{}", self.contents[row][col]);
                }
                // we need to go down a line after we are done
                print!("\n");
            }
        }
    }

    // write some text to the surface
    impl Write for Surface {
        fn write(&mut self, source: String, dest: Coordinate) {
            let mut i: u32 = 0;
            for char in source.chars() {
                // if we go out of bounds, draw_char won't change the surface.
                self.draw_char(Coordinate::new(dest.x + i, dest.y), char);
                i += 1;
            }
        }
    }

    impl DrawLine for Surface {
        fn draw_line(&mut self, a: Coordinate, b: Coordinate, line_char: char) {
            let mut working_a: Coordinate;
            let mut working_b: Coordinate;
            // this was the best way I could find to swap coordinates
            if b.x < a.x {
                working_a = b;
                working_b = a;
            }
            else {
                working_a = a;
                working_b = b;
            }
            // check if we are vertical
            if a.x == b.x {
                // handle vertical line case
                // first, make sure they are in the proper order
                if b.y < a.y {
                    working_a = b;
                    working_b = a;
                }
                for i in working_a.y..working_b.y {
                    self.draw_char(Coordinate::new(a.x, i), line_char);
                }
                return
            }
            // get our slope
            //                          All this casting is necessary because one or both of these values could (and in
            //                              in some circumstances, should) be negative. Obviously, an unsigned value
            //                              doesn't allow for this, so we temporarily convert to a signed integer.
            let m: f64 = (working_a.y as f64 - working_b.y as f64) / (working_a.x as f64 - working_b.x as f64);
            println!("slope: {}", m);
            // y coordinate needs to be remembered as a floating point number
            let mut target_y: f64 = working_a.y.into();
            // this is where the magic happens:
            //  we draw a char at the target point and then increase the x value by 1 and 
            //  increase (or decrease, depending on the sign) the y value by the slope 
            for i in working_a.x..working_b.x+1 {
                //     this has to be rounded in a non-truncating way \/
                self.draw_char(Coordinate::new(i, target_y as u32), line_char);
                target_y += m;
                println!("target_y: {}", target_y);
            }
        }

    }


    pub fn blit(source: Surface, dest: &mut Surface, topleft: Coordinate) {
        // have to do some quick conversion here so that we can index vectors later
        let y_offset = usize::try_from(topleft.y).unwrap();
        let x_offset = usize::try_from(topleft.x).unwrap();
        for row in 0..source.contents.len() {
            for col in 0..source.contents[row].len() {
                // to make sure we don't blit out of bounds
                // leftward and topward bounds aren't a problem because coordinates are always positive
                if dest.verify_point(&Coordinate::new(
                    (col + x_offset).try_into().unwrap(), 
                    (row + y_offset).try_into().unwrap()
                )) {
                    dest.contents[row + y_offset][col + x_offset] = source.contents[row][col];
                }
            }
        }
    }
}