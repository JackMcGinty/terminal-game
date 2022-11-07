pub mod test {
    use crate::surface::surface::{Surface, blit, Write, DrawLine};
    use crate::surface::surface::{
        Clear, Fill, Display, DrawChar
    };
    use crate::coordinate::coordinate::Coordinate as C;
    use crate::screen::screen::Screen;
    use termion::color::*;
    pub fn run_tests() {
        println!("=============== TESTING ================");
        // change this for testing conditions
        surface_test();
        println!("========== TESTING COMPLETE ============");
    }

    fn coordinate_test() {
        let mut test_coord = C::new(3, 4);
        println!("the point is at: {}", test_coord);
        println!("moving to (8, 10)");
        test_coord.reposition(8, 10);
        println!("the point is at: {}", test_coord);
        
    }

    fn surface_test() {
        println!(" == testing Surface == ");
        let mut test_surf = Surface::new(16, 8, '.');
        println!("original surface: ");
        test_surf.display();
        println!("attempting to draw line from (0, 0) to (15, 0)");
        test_surf.draw_line(C::new(0, 0), C::new(15, 0), 'a');
        test_surf.display();
        println!("attempting to draw line from (1, 1) to (4, 4)");
        test_surf.draw_line(C::new(1, 1), C::new(4, 4), 'b');
        test_surf.display();
        println!("attempting to draw line from (11, 3) to (5, 3)");
        test_surf.draw_line(C::new(11, 3), C::new(5, 3), 'c');
        test_surf.display();
        println!("attempting to draw line from (7, 7) to (7, 2)");
        test_surf.draw_line(C::new(7, 7), C::new(7, 2), 'd');
        test_surf.display();
        println!("attempting to draw line from (2, 5) to (9, 1)");
        test_surf.draw_line(C::new(2, 5), C::new(9, 1), 'e');
        test_surf.display();
        println!(" == end of Surface tests == ")
    }

    fn screen_test() {
        println!(" == testing Screen ==");
        let mut test_screen = Screen::new();
        test_screen.draw_char(C::new(5, 8), '#');
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
        - drawing line to surface... check 
    ... add more as necessary
*/