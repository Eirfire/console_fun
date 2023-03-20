use std::thread::sleep;
use std::time::Duration;

use ball::Ball;

mod ball;

fn main() {
    

    loop {
        
        sleep(Duration::from_millis(33));
    }
}


fn Turn() {
    ball::Ball::move_ball(&mut self);
    
}