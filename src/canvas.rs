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

    pub fn prepare_background(&self) -> String {
        let mut out = String::new();
        for _ in 0..self.height {
            for _ in 0..self.width {
                out.push(self.background);
            }
        }
        out
    }

    pub fn draw_string(&self, x: usize, y: usize, new: &str, s: &mut String) {
        let mut offset = 0;
        for ch in new.chars() {
            self.set_char_at(x + offset, y, ch, s);
            offset += 1;
        }
    }

    pub fn draw_border(&self, component: &Component, s: &mut String) {
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
        self.set_char_at(
            x, 
            y, 
            self.get_border_char(x, y), 
            s
        )
    } 

    fn set_char_at(&self, x: usize, y: usize, c: char, s: &mut String) {
        let char_index = y * self.width + x;
        let (pos, old_char) = s.char_indices().nth(char_index).unwrap();
        let byte_count = old_char.len_utf8();
        let char_str = &c.to_string();
        s.replace_range(pos..pos + byte_count, char_str)
    }

    fn get_border_char(&self, x: usize, y: usize) -> char {
        let top_neighbor = if y > 0 {
            self.is_border(x, y-1)
        } else {
            None
        };

        let left_neighbor = if x > 0 {
            self.is_border(x-1, y)
        } else {
            None
        };

        let bottom_neighbor = if y < self.height-1 {
            self.is_border(x, y+1)
        } else {
            None
        };

        let right_neighbor = if x < self.width-1 {
            self.is_border(x+1, y)
        } else {
            None
        };

        border::get_tile(
            self,
            top_neighbor, 
            left_neighbor, 
            bottom_neighbor, 
            right_neighbor
        )
    }

    fn is_border(&self, x: usize, y: usize) -> Option<bool> {
        for component in &self.components {
            let sx = component.x;
            let ex = component.x + component.width;
            let sy = component.y;
            let ey = component.y + component.height;

            if sx <= x && x < ex && (sy == y || ey-1 == y) {
                return Some(component.bold);
            } else if sy <= y && y < ey && (sx == x || ex-1 == x) {
                return Some(component.bold);
            } 
        }
        return None
    }

}
