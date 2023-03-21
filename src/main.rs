use std::thread::sleep;
use std::time::Duration;
use ball::{Ball, Point};
use rand::{self, random};
mod ball;


struct Game {
    screen: Vec<Vec<char>>,
    ball: Ball,
    random: rand::rngs::ThreadRng,
}

impl Game {
    fn new(rows: usize, cols: usize) -> Game {
        let screen = "";
        let ball = Ball::new(
            Point { x: rand::random::<usize>() % rows, y: rand::random::<usize>() % cols }, 
            rows, 
            cols
        )
    }



    fn fill_screen(c: char) {
        for i in 0..self::screen.len(0) {
            for j in 0..self::screen[i].len(1) {
                self::screen[i, j] = c;
            }
        }
    }
}

// impl Game {
//     fn new(rows: usize, cols: usize) -> Game {
//         let  screen = vec![vec!['_'; rows]; cols];
//         let  ball = Ball::new(
//             Point {
//                 x: rand::random::<usize>() % cols,
//                 y: rand::random::<usize>() % rows,
//             },
//             cols,
//             rows,
//         );
//         // let random = rand::thread_rng();
//         Game { screen, ball }
//     }

//     // async fn start(&mut self) {
//     //     loop {
//     //         self.turn();
//     //         sleep(Duration::from_millis(33));
//     //     }
//     // }

        // fn fill_screen(&mut self, c: char) {
        //     for i in 0..self.screen.len() {
        //         for j in 0..self.screen[i].len() {
        //             self.screen[i][j] = c;
        //         }
        //     }
        // }

//     fn turn(&mut self) {
//         self.ball.move_ball();
//         for i in 0..self.screen.len() {
//             for j in 0..self.screen[i].len() {
//                 println!("\x1b[{};{}H", i, j);

//                 if self.ball.position.x == i.try_into().unwrap() && self.ball.position.y == j.try_into().unwrap() {
//                     println!("{}", self.ball.ball_character);
//                 } else {
//                     println!("{}", self.screen[i][j]);
//                 }
//             }
//         }
//     }
// }

fn main() {
    let mut game = Game::new(20, 20);
    {
        let ref mut this = game;
        loop {
            this.turn();
            sleep(Duration::from_millis(33));
        }
    };
}