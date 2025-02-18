use std::{fmt::write, io::{self, Write}};

fn flush() {
    io::stdout().flush().unwrap();
}

pub fn enter_alter_scr_buff() {
    write!(io::stdout(), "\x1B[?1049h").unwrap();
    flush();
}

pub fn wait_for_exit() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
}

pub fn exit_alter_scr_buff() {
    write!(io::stdout(), "\x1B[?1049l").unwrap();
    flush();
}

pub fn mv_cur(row: usize, col: usize) {
    write!(
        io::stdout(),
        "\x1b[{};{}H", row, col
    ).unwrap();
    flush();
}
