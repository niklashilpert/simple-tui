use std::{cmp::max, io::{self, stdout, Write}};

use crossterm::{cursor, event::{self, Event, KeyCode}, execute, queue, terminal};

use crate::drawing::{render, Component};




pub struct Canvas {
    components: Vec<Component>,
    width: u16,
    height: u16,

}

pub fn start() -> io::Result<()> {
    let mut out = stdout();

    terminal::enable_raw_mode()?;    

    execute!(out, 
        terminal::EnterAlternateScreen, 
        //cursor::DisableBlinking, 
        //cursor::Hide
    )?;

    let main_window_size = (30, 20);
    let size = terminal::size()?;

    let main_window_tl = (
        if size.0 < main_window_size.0 {0} else {size.0 / 2 - main_window_size.0 / 2},
        if size.1 < main_window_size.1 {0} else {size.1 / 2 - main_window_size.1 / 2},
    ); 
    let components = vec![
        Component {
            width: main_window_size.0,
            height: main_window_size.1,
            x: main_window_tl.0,
            y: main_window_tl.1,
            title: String::from("SimpleTUI"),
            bold: false,
        }
    ];

    render(&mut out, &components)?;
    out.flush()?;


    handle_events(&mut out, &components)?;
    execute!(out, 
        cursor::Show, 
        cursor::EnableBlinking,
        terminal::LeaveAlternateScreen, 
    )?;

    terminal::disable_raw_mode()?;

    Ok(())
}


fn handle_events(out: &mut impl Write, components: &Vec<Component>) -> io::Result<()> {
    loop {
        match event::read()? {
            Event::Key(event) => {
                match event.code {
                    KeyCode::Char('q') => {
                        return Ok(());
                    },
                    KeyCode::Up => {
                        execute!(out, cursor::MoveUp(1))?;
                    },
                    KeyCode::Down => {
                        execute!(out, cursor::MoveDown(1))?;
                    },
                    KeyCode::Left => {
                        execute!(out, cursor::MoveLeft(1))?;
                    },
                    KeyCode::Right => {
                        execute!(out, cursor::MoveRight(1))?;
                    },
                    _ => {

                    }
                }
            },
            Event::Resize(width, height) => {
                queue!(out, terminal::Clear(terminal::ClearType::All))?;
                render(out, &components)?;
                out.flush()?;
            }
            _ => {},
        }
    }
}