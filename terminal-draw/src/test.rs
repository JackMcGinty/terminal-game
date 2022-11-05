pub mod test {
    use crate::surface::surface::Surface;
    use crate::surface::surface::{
        Clear, Fill, Display, DrawChar
    };
    use crate::coordinate::coordinate::Coordinate;
    use crate::screen::screen::Screen;

    pub fn run_tests() {
        println!("=============== TESTING ================");
        // change this for testing conditions
        screen_test();
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
        let mut test_surface = Surface::new(4, 4, '*');
        println!("original surface");
        test_surface.display();
        println!("attempting to change (0, 0) to a");
        test_surface.draw_char(Coordinate::new(0, 0), 'a');
        test_surface.display();
        println!("attempting to change (3, 3) to b");
        test_surface.draw_char(Coordinate::new(3, 3), 'b');
        test_surface.display();
        // now the tricky bit
        println!("attempting to change (3, 0) to c");
        test_surface.draw_char(Coordinate::new(3, 0), 'c');
        test_surface.display();
        println!("attempting to change (1, 2) to d");
        test_surface.draw_char(Coordinate::new(1, 2), 'd');
        test_surface.display();
        println!("attempting to change an invalid coordinate ((6, 4) in this case) to e\nsurface should be unchanged");
        test_surface.draw_char(Coordinate::new(6, 4), 'e');
        test_surface.display();
        
        println!(" == end of Surface tests == ")
    }

    fn screen_test() {
        println!(" == testing Screen ==");
        let mut test_screen = Screen::new();
        test_screen.fill('&');
        test_screen.display();
        println!("== end of Screen tests == ")
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
    ... add more as necessary
*/