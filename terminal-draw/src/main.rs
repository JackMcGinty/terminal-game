// Just does testing

// we still need our modules declared though
mod coordinate;
mod surface;
mod screen;
// testing module
mod test;
use test::test::run_tests;

fn main() {
    run_tests();
}
