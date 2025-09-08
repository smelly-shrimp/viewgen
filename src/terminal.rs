use std::io::{self, Stdin, Write};

pub fn flush() {
    io::stdout().flush().unwrap();
}

pub fn enter_alter_scr() {
    write!(io::stdout(), "\x1B[?1049h").unwrap();
    flush();
}

pub fn input(stdin: &Stdin, input: &mut String) {
    input.clear();
    stdin.read_line(input).unwrap();
}

pub fn exit_alter_scr() {
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
