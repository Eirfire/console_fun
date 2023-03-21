#[derive(Debug, Clone, Copy)]
pub struct Point {
    pub x: usize,
    pub y: usize,
}

impl Point {
    pub fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }
}

pub struct Ball {
    pub position: Point,
    pub ball_going_down: bool,
    pub ball_going_right: bool,
    pub screen_x: usize,
    pub screen_y: usize,
    pub ball_character: char,
}

impl Ball {
    pub fn new(position: Point, screen_x: usize, screen_y: usize) -> Self {
        Ball {
            position,
            ball_going_down: true,
            ball_going_right: true,
            screen_x,
            screen_y,
            ball_character: '+',
        }
    }

    pub fn move_ball(&mut self) {
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
        
        let ball_position = Point::new(self.position.x + ball_x, self.position.y + ball_y);

        if ball_position.y == 0 || ball_position.y == (self.screen_y -1).try_into().unwrap() {
            self.ball_going_down = !self.ball_going_down
        } 

        if ball_position.x == 0 || ball_position.x == (self.screen_x -1).try_into().unwrap() {
            self.ball_going_right = !self.ball_going_right
        }
    }
}