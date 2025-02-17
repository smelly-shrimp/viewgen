struct View {
    buff: [[char; 80]; 24],
    height: u8,
}

impl View {
    fn draw(&self) {
        for row in self.buff.iter() {
            for &elm in row.iter() {
                print!("{elm}");
            }

            print!("\n")
        }
    }
}

fn main() {
    let foo = View {
        buff: [['.'; 80]; 24],
        height: 20,
    };

    foo.draw();
}
