use crate::canvas::{Canvas, Component};

mod term_io;
mod canvas;

fn main() {
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
                x: 0,
                y: 0,
                width: 5,
                height: 5,
            }
        ]
    };

    print!("{}", canvas.render());

    _ = term_io::flush();

    term_io::await_input();
}

