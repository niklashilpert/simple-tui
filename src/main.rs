use std::{thread, time::Duration};

use crate::canvas::{Canvas, Component};

mod term_io;
mod canvas;
mod border;

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

    let mut canvas = Canvas {
        width: term_width as usize,
        height: term_height as usize,
        background: ' ',
        components: vec![
            Component {
                title: String::from("Window"),
                x: 0,
                y: 0,
                width: 20,
                height: 5,
                bold: false,
            },
            Component {
                title: String::from("Window 2"),
                x: 25,
                y: 4,
                width: 15,
                height: 6,
                bold: true,
            }
        ]
    };

    loop {
        let terminal_size = termion::terminal_size().unwrap();

        canvas.width = terminal_size.0 as usize;
        canvas.height = terminal_size.1 as usize;
        println!();

        print!("{}", canvas.render());

        print!("{esc}c", esc = 27 as char);
        thread::sleep(Duration::from_millis(30))
    }
}

