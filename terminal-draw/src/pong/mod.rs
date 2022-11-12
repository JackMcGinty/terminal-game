mod game_classes;

use crate::{screen::{self, screen::Screen}, surface::surface::{Display, Surface, Write}, coordinate::coordinate::Coordinate};

use self::game_classes::{ball::Ball, velocity::Velocity};
use std::{thread, time};

use crate::surface::surface::Clear;

use rand::{self, Rng};

extern crate termion;

pub fn play() {
    let mut screen = screen::screen::Screen::new();
    let mut ball_vec: Vec<Ball> = Vec::new();
    for _i in 0..=rand::thread_rng().gen_range(1..9) {
        let new_ball = Ball::new(random_position(screen.surf.clone()), random_velocity());
        ball_vec.push(new_ball);
    }
    let thanks = true;
    if thanks {
        let mut word_surf = Surface::new(20, 1, '*');
        word_surf.write("thanks for watching!".to_string(), Coordinate::new(0, 0));
        let word_ball = Ball {
            position: Coordinate::new(5, 5),
            velocity: Velocity::new(2, 1),
            surface: word_surf
        };
        ball_vec.push(word_ball);
    }
    // game loop
    let mut i = 0;
    loop {
        // draw stuff
        for i in 0..ball_vec.len() {
            ball_vec[i].draw(&mut screen.surf);
        }
        // screen clearing isn't working so well on non-vscode terminal, so this is here for backup
        std::process::Command::new("clear").status().expect("couldn't");
        
        screen.display();
        screen.clear();
        for i in 0..ball_vec.len() {
            ball_vec[i].apply_velocity();
            keep_ball_on_screen(&mut ball_vec[i], &screen);
        }

        // sleep
        thread::sleep(time::Duration::from_millis(75));
        i += 1;
        if i > 800 {
            break;
        }
    }
}

fn keep_ball_on_screen (ball: &mut Ball, screen: &Screen) {
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

fn random_velocity () -> Velocity {
    Velocity::new(rand::thread_rng().gen_range(-4..4), rand::thread_rng().gen_range(-2..2))
}

fn random_position (target_surf: Surface) -> Coordinate {
    Coordinate::new(rand::thread_rng().gen_range(1..target_surf.width) as i32, rand::thread_rng().gen_range(1..target_surf.height) as i32)
}