pub mod test {
    use crate::surface::surface::{Surface, blit};
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
        // time for some blit testing
        let mut surf_a = Surface::new(8, 4, 'a');
        let surf_b = Surface::new(4, 2, 'b');
        println!("surface a:");
        surf_a.display();
        println!("surface b:");
        surf_b.display();
        println!("attempting to blit b to a at (6, 1)");
        blit(surf_b, &mut surf_a, Coordinate::new(6, 1));
        surf_a.display();
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
    ... add more as necessary
*/