use std::io::{self, Stdin};

use crate::view::View;

mod term;
mod view;

fn get_val(stdin: &Stdin, input: &mut String, msg: &str) -> usize {
    print!("{}: ", msg);
    term::flush();
    term::input(stdin, input);

    input.trim().parse().unwrap_or_else(|_| {
        println!("Cannot parse value to number, try again");
        get_val(stdin, input, msg)
    })
}

fn main() {
    let stdin = io::stdin();
    let mut input = String::new();

    let height = get_val(&stdin, &mut input, "(min) height");
    let spikeness = get_val(&stdin, &mut input, "spikeness");

    let mut view = View::new();
    view.gen(height, spikeness);
    view.draw(&stdin, &mut input)
}
