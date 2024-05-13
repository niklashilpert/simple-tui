use std::io::{self, stdout, Write};

use crossterm::{cursor::{self, MoveDown, MoveLeft, MoveRight, MoveUp, Show}, event::{read, Event, KeyCode, KeyEvent}, execute, queue, terminal};

use crate::drawing::{Component, render};

mod term_io;
mod canvas;
mod border;
mod drawing;
mod tui;
mod widget;

fn main() { 
    //test1();
    _ = tui::start();
}



/*fn test1() {
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
                title: String::from("SimpleTUI"),
                x: 0,
                y: 0,
                width: term_width as usize,
                height: term_height as usize,
                bold: false,
            },
            Component {
                title: String::from("Window"),
                x: 0,
                y: 0,
                width: 15,
                height: 6,
                bold: true,
            },
            Component {
                title: String::from(""),
                x: 12,
                y: 3,
                width: 15,
                height: 6,
                bold: false,
            },
            Component {
                title: String::from(""),
                x: 13,
                y: 4,
                width: 13,
                height: 4,
                bold: false,
            }
        ]
    };

    loop {
        let terminal_size = termion::terminal_size().unwrap();

        canvas.width = terminal_size.0 as usize;
        canvas.height = terminal_size.1 as usize;

        
        let mut screen = stdout().into_alternate_screen().unwrap();
        _= write!(screen, "{}", canvas.render());
        //print!("{}", canvas.render());
        
        

        //thread::sleep(Duration::from_millis(30));
        term_io::await_input();
    }
}*/

