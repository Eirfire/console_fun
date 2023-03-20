mod point;

pub struct Ball {
    position: Point,
    ball_going_down: bool,
    ball_going_right: bool,
    screen_x: i32,
    screen_y: i32,
    ball_character: char,
}

impl Ball {
    fn new(position: Point, screen_x: i32, screen_y: i32) -> Self {
        Ball {
            position,
            ball_going_down: true,
            ball_going_right: true,
            screen_x,
            screen_y,
            ball_character: '+',
        }
    }

    fn move_ball(&mut self) {
        let mut ball_x = 0;
        let mut ball_y = 0;
        if self.ball_going_down {
            ball_y += 1;
        } else {
            ball_y -= 1;
        }
        if self.ball_going_right {
            ball_x += 1;
        } else {
            ball_x -= 1;
        }
        
        let mut ball_position = Point::new(self.position.0 + ball_x, self.position.1 + ball_y);

        if ball_position.1 == 0 || ball_position.1 == screen_y -1 {
            ball_going_down = !ball_going_down
        } 

        if ball_position.0 == 0 || ball_position.0 == screen_x -1 {
            ball_going_right = !ball_going_right
        }
    }
}