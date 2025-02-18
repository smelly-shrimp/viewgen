use rand::Rng;
use terminal::mv_cur;
use text_io::read;

mod terminal;

const COL_LEN: usize = 24;

struct View {
    buff: [usize; 80],
    height: usize,
    spikeness: usize,
}

impl View {
    fn new(mut height: usize, mut spikeness: usize) -> View {
        if height + spikeness > COL_LEN {
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

    fn draw(&self) {
        terminal::enter_alter_scr_buff();

        let mut col = 1;
        for h in self.buff {
            let blank = COL_LEN - h - self.height - self.get_chunk();

            for row_i in 0..blank {
                mv_cur(row_i + 1, col);
                print!(".");
            }

            for row_i in blank..COL_LEN {
                mv_cur(row_i + 1, col);
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

fn get_values() -> (usize, usize) {
    print!("(min) Height: ");
    let height: usize = read!();
    
    print!("Spikeness: ");
    let spikeness: usize = read!();

    (height, spikeness)
}

fn main() {
    let vals = get_values();

    let foo = View::new(vals.0, vals.1);

    foo.draw();
}
