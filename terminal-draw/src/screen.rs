pub mod screen {
    // borrow traits from the suface file
    use crate::surface::surface::{
        Clear, Fill, DrawChar, 
        Display, VerifyPoint, Write
    };
    // we also need the Surface class
    use crate::surface::surface::Surface;
    // Termion will be helpful with input as well
    use termion;
    // pretty much the same thing as surface except:
    //  It displays differently
    //  it is always the size of the screen
    //  and it's default character is always ' '.
    pub struct Screen {
        surf: Surface,
        width: u32,
        height: u32,
    }
    impl Screen {
        // we need a constructor here...
        //  and that's pretty much it. 
        // Trait implementations take care of the rest.
        pub fn new() -> Self {
            let terminal_dimensions = termsize::get().unwrap();
            Self {
                surf: Surface::new(
                    terminal_dimensions.cols.into(),
                    terminal_dimensions.rows.into(), 
                    ' '
                ),
                width: terminal_dimensions.cols.into(),
                height: terminal_dimensions.rows.into()
            }
        }
    }
    
    impl Clear for Screen {
        fn clear(&mut self) {
            // most of the trait impls will follow this pattern
            //  According to the website, Rust uses composition-based
            //  inheritance, and so these impls are really just 
            //  wrappers for the internal contents.
            self.surf.clear();
        }
    }
    impl Fill for Screen {
        fn fill(&mut self, new_char: char) {
            self.surf.fill(new_char);
        }
    }
    impl DrawChar for Screen {
        fn draw_char(&mut self, dest_coord: crate::coordinate::coordinate::Coordinate, dest_char: char) {
            self.surf.draw_char(dest_coord, dest_char);
        }
    }
    impl VerifyPoint for Screen {
        fn verify_point(&self, test_point: &crate::coordinate::coordinate::Coordinate) -> bool {
            self.surf.verify_point(test_point) // no ; because we are returning
        }
    }
    impl Write for Screen {
        fn write(&mut self, source: String, dest: crate::coordinate::coordinate::Coordinate) {
            self.surf.write(source, dest);
        }
    }
    impl Display for Screen {
        fn display(&self) {
            // first clear and go to the top left of the screen
            //  Thanks, Termion
            print!("{}{}", termion::clear::All, termion::cursor::Goto(1, 1));
            // why not (0, 0)? Something to do with ansii-escape codes
            // the screen is clear so now we call the surf display
            // maybe? I might need to change it
            self.surf.display();
        }
    }
}