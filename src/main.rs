use crate::canvas::{Canvas, Component};

mod term_io;
mod canvas;

fn main() {

    let test = String::from("01234567890123456789");
    let mut chars_indices = test.char_indices();
    let c = chars_indices.nth(3).unwrap();
    let c2 = chars_indices.nth(10).unwrap();

    println!("{} {}", c.1, c2.1);

    let terminal_size = termion::terminal_size().unwrap();

    let term_width = terminal_size.0;
    let term_height = terminal_size.1;

    println!("{}, {}", term_width, term_height);

    let canvas = Canvas {
        width: term_width,
        height: term_height,
        background: ' ',
        components: vec![
            Component {
                title: String::from("Window"),
                x: 0,
                y: 0,
                width: 20,
                height: 5,
            }
        ]
    };

    print!("{}", canvas.render());

    _ = term_io::flush();

    term_io::await_input();
}

