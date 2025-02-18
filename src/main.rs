use text_io::read;

mod terminal;
mod view;

fn get_values() -> (usize, usize) {
    print!("(min) Height: ");
    let height: usize = read!();

    print!("Spikeness: ");
    let spikeness: usize = read!();

    (height, spikeness)
}

fn main() {
    let vals = get_values();

    let foo = view::View::new(vals.0, vals.1);

    foo.draw();
}
