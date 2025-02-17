mod terminal;

struct View {
    buff: [[char; 80]; 24],
    height: u8,
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
}

fn main() {
    let foo = View {
        buff: [['.'; 80]; 24],
        height: 20,
    };

    foo.draw();
}
