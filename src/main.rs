use rand::Rng;
use terminal::mv_cur;

mod terminal;

const COL_LEN: usize = 24;

struct View {
    buff: [usize; 80],
    height: usize,
}

type Chunk = [char; 24];

impl View {
    fn draw(&self) {
        terminal::enter_alter_scr_buff();

        let mut col = 1;
        for h in self.buff {
            let blank = COL_LEN - h - self.height;

            for row_i in 0..blank {
                mv_cur(row_i + 1, col);
                print!(".");
            }

            for row_i in COL_LEN - h - self.height..COL_LEN {
                mv_cur(row_i + 1, col);
                print!("#");
            }

            col += 1;
        }

        terminal::wait_for_exit();
        terminal::exit_alter_scr_buff();
    }

    fn get_chunk(&self) -> usize {
        let max_val = COL_LEN - self.height;

        let h = rand::rng().random_range(0..=max_val);

        COL_LEN - (self.height + h)
    }
}

fn main() {
    let foo = View {
        buff: [0; 80],
        height: 8,
    };

    foo.draw();
}
