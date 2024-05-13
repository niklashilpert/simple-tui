use std::{char, io::{self, Write}};

use crossterm::{cursor::MoveTo, queue, style};

use crate::border;

pub struct Component {
    pub title: String,
    pub x: u16,
    pub y: u16,
    pub width: u16,
    pub height: u16,
    pub bold: bool,
}

pub fn render(out: &mut impl Write, components: &Vec<Component>) -> io::Result<()> {
    for component in components {
        render_component(out, component, components)?;
    }
    Ok(())
}

fn render_component(out: &mut impl Write, component: &Component, components: &Vec<Component>) -> io::Result<()> {
    let sx = component.x;
    let ex = component.x + component.width;
    let sy = component.y;
    let ey = component.y + component.height;
    
    for x in sx..ex {
        set_border(out, x, sy, components)?;
        set_border(out, x, ey-1, components)?;
    }

    for y in sy+1..ey {
        set_border(out, sx, y, components)?;
        set_border(out, ex-1, y, components)?;
    }



    draw_string(out, sx+2, sy, &component.title)?;

    Ok(())
}



fn draw_string(out: &mut impl Write, x: u16, y: u16, s: &str) -> io::Result<()> {
    queue!(out, MoveTo(x, y))?; 
    for char in s.chars() {
        queue!(out, style::Print(char))?
    }
    Ok(())
}

fn set_border(out: &mut impl Write, x: u16, y: u16, components: &Vec<Component>) -> io::Result<()> {
    queue!(out, MoveTo(x, y), style::Print(get_border_char(x, y, components)))
} 


fn get_border_char(x: u16, y: u16, components: &Vec<Component>) -> char {        
    let mut connections_map = 0b0000;
    let mut bold_map = 0b0000;

    for component in components {
        let directions_map = component.get_border_directions(x, y);

        connections_map |= directions_map;
        bold_map |= if component.bold { directions_map } else { 0 };
    }

    border::get_tile(connections_map as i32, bold_map as i32, ' ')
}

impl Component {
    fn get_border_directions(&self, x: u16, y: u16) -> usize {
        let sx = self.x;
        let ex = sx + self.width - 1;
        let sy = self.y;
        let ey = sy + self.height - 1;

        // 0 = false; 1 = true
        // Syntax: top left bottom right

        return if (x, y) == (sx, sy) {
            0b0011
        } else if (x, y) == (ex, sy) {
            0b0110
        } else if (x, y) == (ex, ey) {
            0b1100
        } else if (x, y) == (sx, ey) {
            0b1001
        } else if (y == sy || y == ey) && sx <= x && x < ex {
            0b0101
        } else if (x == sx || x == ex) && sy <= y && y < ey {
            0b1010
        } else {
            0b0000
        };
    }
}
