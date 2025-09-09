use std::io::Stdin;

use crate::term;
use rand::Rng;

const COLH: usize = 24;

pub struct View {
    buff: [usize; 80],
}

impl View {
    pub fn new() -> View {
        View { buff: [0; 80] }
    }

    pub fn gen(&mut self, mut height: usize, mut spikeness: usize) {
        let sum = height + spikeness;

        if sum > COLH {
            let height_part = height as f64 / sum as f64;

            height = (COLH as f64 * height_part) as usize;
            spikeness = COLH - height;
        }

        for col in self.buff.iter_mut() {
            *col = rand::rng().random_range(height..=height + spikeness);
        }
    }

    pub fn draw(&self, stdin: &Stdin, input: &mut String) {
        term::enter_alter_scr();

        for (col, height) in self.buff.iter().enumerate() {
            let blank = COLH - height;

            for row in 0..blank {
                term::print(row, col, '.');
            }

            for row in blank..COLH {
                term::print(row, col, '#');
            }
        }

        term::flush();
        term::input(stdin, input);
        term::exit_alter_scr();
    }
}
