mod game_classes;

use crate::{screen::{self, screen::Screen}, surface::surface::{blit, Display, Write, DrawChar, VerifyPoint}, coordinate::coordinate::Coordinate};

use self::game_classes::ball::Ball;
use std::{thread, time};

use crate::surface::surface::Clear;

pub fn play() {
    let mut demo_screen = screen::screen::Screen::new();
    let mut demo_ball = Ball::new();
    let mut i = 0;
    // game loop
    loop {
        i += 1;
        demo_ball.draw(&mut demo_screen.surf);
        // screen clearing isn't working so well on non-vscode terminal, so this is here for backup
        std::process::Command::new("clear").status().expect("couldn't");
        // debug
        let ball_position_is_valid = demo_screen.surf.verify_point(&demo_ball.position);
        demo_screen.write(format!("({},{}) i = {} valid position = {}", demo_ball.position.x, demo_ball.position.y, i, ball_position_is_valid).to_string(), Coordinate::new(0, 0));
    
        demo_screen.display();
        demo_screen.clear();
        demo_ball.apply_velocity();
        keep_ball_on_screen(&mut demo_ball, &demo_screen);

        // sleep
        //  for a game this simple, this should be fine.
        thread::sleep(time::Duration::from_millis(75));
        if i > 200 {
            break;
        }
    }
}

fn keep_ball_on_screen (ball: &mut Ball, screen: &Screen) {
    // this will probably have to change to handle scoring
    // left bound
    if ball.position.x <= 0 && ball.velocity.x < 0 {
        ball.bounce(true, false);
    }
    // top bound 
    if ball.position.y <= 0 && ball.velocity.y < 0 {
        ball.bounce(false, true);
    }
    // right bound
    if (ball.position.x + ball.surface.width as i32) >= (screen.surf.width).try_into().unwrap() && ball.velocity.x > 0 {
        ball.bounce(true, false);
    }
    // bottom bound
    if (ball.position.y + ball.surface.height as i32) >= (screen.surf.height).try_into().unwrap() && ball.velocity.y > 0 {
        ball.bounce(false, true);
    }

}