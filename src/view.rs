use rand::Rng;
use crate::terminal;

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

    pub fn draw(&self) {
        terminal::enter_alter_scr_buff();

        let mut col = 1;
        for h in self.buff {
            let blank = COL_LEN - h - self.height - self.get_chunk();

            for row_i in 0..blank {
                terminal::mv_cur(row_i + 1, col);
                print!(".");
            }

            for row_i in blank..COL_LEN {
                terminal::mv_cur(row_i + 1, col);
                print!("#");
            }

            col += 1;
        }

        terminal::wait_for_exit();
        terminal::exit_alter_scr_buff();
    }

    fn get_chunk(&self) -> usize {
        rand::rng().random_range(0..=self.spikeness)
    }
}
