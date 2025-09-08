use std::io::Stdin;

use rand::Rng;
use crate::term;

const COL_LEN: usize = 24;

pub struct View {
    buff: [usize; 80],
    height: usize,
    spikeness: usize,
}

impl View {
    pub fn new(mut height: usize, mut spikeness: usize) -> View {
        let sum = height + spikeness;

        if sum > COL_LEN {
            let out_count = height + spikeness - COL_LEN;
            let pseudo_half = out_count / 2;

            height -= pseudo_half;
            spikeness -= out_count - pseudo_half;
        }

        View {
            buff: [0; 80],
            height,
            spikeness,
        }
    }

    pub fn draw(&self, stdin: &Stdin, input: &mut String) {
        term::enter_alter_scr();

        let mut col = 1;
        for h in self.buff {
            let blank = COL_LEN - h - self.height - self.get_chunk();

            for row_i in 0..blank {
                term::mv_cur(row_i + 1, col);
                print!(".");
            }

            for row_i in blank..COL_LEN {
                term::mv_cur(row_i + 1, col);
                print!("#");
            }

            col += 1;
        }

        term::input(stdin, input);
        term::exit_alter_scr();
    }

    fn get_chunk(&self) -> usize {
        rand::rng().random_range(0..=self.spikeness)
    }
}
