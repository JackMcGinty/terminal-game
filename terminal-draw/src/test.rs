pub mod test {
    use crate::surface::surface::{Surface, blit, Write};
    use crate::surface::surface::{
        Clear, Fill, Display, DrawChar
    };
    use crate::coordinate::coordinate::Coordinate;
    use crate::screen::screen::Screen;

    pub fn run_tests() {
        println!("=============== TESTING ================");
        // change this for testing conditions
        surface_test();
        println!("========== TESTING COMPLETE ============");
    }

    fn coordinate_test() {
        let mut test_coord = Coordinate::new(3, 4);
        println!("the point is at: {}", test_coord);
        println!("moving to (8, 10)");
        test_coord.reposition(8, 10);
        println!("the point is at: {}", test_coord);
        
    }

    fn surface_test() {
        println!(" == testing Surface == ");
        // write method!
        let mut test_surf = Surface::new(16, 8, '@');
        println!("surface: ");
        test_surf.display();
        println!("attempting to write the gettysburg address to the surface @ (1, 5)");
        test_surf.write("Four score and seven years ago our father brought forth on this continent, a new nation, conceived in Liberty, and dedicated to the proposition that all men are created equal.".to_string(), Coordinate::new(1, 5));
        test_surf.display();
        println!(" == end of Surface tests == ")
    }

    fn screen_test() {
        println!(" == testing Screen ==");
        let mut test_screen = Screen::new();
        test_screen.draw_char(Coordinate::new(5, 8), '#');
        test_screen.display();
        println!("== end of Screen tests == ");
    }
}

// testing info:
/*
    - Coordinate... Check
    - Surface... 
        - creating and displaying... check
        - filling and clearing... check
        - changing one valid coordinate... check
        - screen displays at the right size... check
        - surface blitting... check
        - writing text to surface... check
    ... add more as necessary
*/