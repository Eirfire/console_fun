use std::io::{stdout, Write};
use std::thread::sleep;
use std::time::Duration;
use rand::Rng;


const SCREEN_ROWS: usize = 10;
const SCREEN_COLS: usize = 100;

struct Ball {
    position: (usize, usize),
    ball_character: char,
    ball_going_down: bool,
    ball_going_right: bool,
    screen_x: usize,
    screen_y: usize,
}

impl Ball {
    fn new(screen_x: usize, screen_y: usize) -> Ball {
        let mut rng = rand::thread_rng();
        Ball {
            Point { x: rand::random::<usize>() % rows, y: rand::random::<usize>() % cols },
            ball_character: '+',
            ball_going_down: true,
            ball_going_right: true,
            screen_x,
            screen_y,
        }
    }

    fn move_ball(&mut self) {
        let mut ballx = 0;
        let mut bally = 0;
        if self.ball_going_down {
            bally += 1;
        } else {
            bally -= 1;
        }
        if self.ball_going_right {
            ballx += 1;
        } else {
            ballx -= 1;
        }

        let new_x = self.position.0 + ballx;
        let new_y = self.position.1 + bally;
        self.position = (new_x, new_y);

        if self.position.1 == 0 || self.position.1 == self.screen_y - 1 {
            self.ball_going_down = !self.ball_going_down;
        }

        if self.position.0 == 0 || self.position.0 == self.screen_x - 1 {
            self.ball_going_right = !self.ball_going_right;
        }
    }
}

struct Game {
    screen: Vec<Vec<char>>,
    ball: Ball,
    random: rand::rngs::ThreadRng,
}

impl Game {
    fn new(rows: usize, cols: usize) -> Game {
        let screen = vec![vec!['_'; rows]; cols];
        let ball = Ball::new(cols, rows);
        Game {
            screen,
            ball,
        }
    }

    fn fill_screen(&mut self, c: char) {
        for i in 0..self.screen.len() {
            for j in 0..self.screen[i].len() {
                self.screen[i][j] = c;
            }
        }
    }

    fn turn(&mut self) {
        self.ball.move_ball();
        let mut stdout = stdout();
        for i in 0..self.screen.len() {
            for j in 0..self.screen[i].len() {
                let mut output_char = self.screen[i][j];
                if (i, j) == self.ball.position {
                    output_char = self.ball.ball_character;
                }
                write!(stdout, "{}", output_char).unwrap();
            }
            write!(stdout, "\n").unwrap();
        }
        stdout.flush().unwrap();
    }

    fn start(&mut self) {
        loop {
            self.turn();
            sleep(Duration::from_millis(33));
        }
    }
}

fn main() {
    let mut game = Game::new(SCREEN_ROWS, SCREEN_COLS);
    game.fill_screen('_');
    game.start();
}
