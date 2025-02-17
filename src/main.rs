use rand::Rng;

mod terminal;

struct View {
    buff: [[char; 80]; 24],
    height: usize,
}

impl View {
    fn draw(&self) {
        terminal::enter_alter_scr_buff();

        for row in self.buff.iter() {
            for &elm in row.iter() {
                print!("{elm}");
            }

            print!("\n")
        }

        terminal::wait_for_exit();
        terminal::exit_alter_scr_buff();
    }

    fn get_chunk(&self) -> [char; 24] {
        let mut res = ['.'; 24];

        let max_val = res.len() - self.height;

        let mut h = rand::rng().random_range(0..=max_val);

        h = res.len() - (self.height + h);

        for i in h..res.len() {
            res[i] = '#';
        }

        res
    }
}

fn main() {
    let foo = View {
        buff: [['.'; 80]; 24],
        height: 8,
    };

    foo.draw();
    println!("{:?}", foo.get_chunk());
}
