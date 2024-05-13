use std::char;

use crate::border;

pub struct Canvas {
    pub width: usize,
    pub height: usize,
    pub components: Vec<Component>,
    pub background: char,
}

pub struct Component {
    pub title: String,
    pub x: usize,
    pub y: usize,
    pub width: usize,
    pub height: usize,
    pub bold: bool,
}

impl Canvas {
    pub fn render(&self) -> String {
        let mut out = self.prepare_background();
        for component in &self.components {
            self.draw_border(component, &mut out);
            self.draw_string(component.x+2, component.y, &component.title, &mut out);
        }
        out
    }

    fn prepare_background(&self) -> String {
        let mut out = String::new();
        for _ in 0..self.height {
            for _ in 0..self.width {
                out.push(self.background);
            }
        }
        out
    }

    fn draw_string(&self, x: usize, y: usize, new: &str, s: &mut String) {
        let mut offset = 0;
        for ch in new.chars() {
            self.set_char_at(x + offset, y, ch, s);
            offset += 1;
        }
    }

    fn draw_border(&self, component: &Component, s: &mut String) {
        let sx = component.x;
        let ex = component.x + component.width;
        let sy = component.y;
        let ey = component.y + component.height;
        
        for x in sx..ex {
            self.set_border(x, sy, s);
            self.set_border(x, ey-1, s)
        }

        for y in sy+1..ey {
            self.set_border(sx, y, s);
            self.set_border(ex-1, y, s);
        }
    }

    fn set_border(&self, x: usize, y: usize, s: &mut String) {
        self.set_char_at(x, y, self.get_border_char(x, y), s)
    } 

    fn set_char_at(&self, x: usize, y: usize, c: char, s: &mut String) {
        let char_index = y * self.width + x;
        let (pos, old_char) = s.char_indices().nth(char_index).unwrap();
        let byte_count = old_char.len_utf8();
        let char_str = &c.to_string();
        s.replace_range(pos..pos + byte_count, char_str)
    }

    fn get_border_char(&self, x: usize, y: usize) -> char {        
        let mut connections_map = 0b0000;
        let mut bold_map = 0b0000;

        for component in &self.components {
            let directions_map = component.get_border_directions(x, y);

            connections_map |= directions_map;
            bold_map |= if component.bold { directions_map } else { 0 };
        }

        border::get_tile(connections_map as i32, bold_map as i32, self.background)
    }

    
}

impl Component {
    fn get_border_directions(&self, x: usize, y: usize) -> usize {
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
