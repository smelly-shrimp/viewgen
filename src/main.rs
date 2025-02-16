struct View {
    buff: [[char; 24]; 80],
    height: u8,
}

fn main() {
    let mut foo = View {
        buff: [[' '; 24]; 80],
        height: 20,
    };
}
